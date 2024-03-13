use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::{error::GitError, hash_writer::HashWriter, limit_reader::LimitReader};

pub fn init() -> Result<(), GitError> {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory");

    Ok(())
}

pub fn cat_file(
    _object_type: &str,
    object_hash: &str,
    _pretty_print: bool,
) -> Result<(), GitError> {
    // 1. Read the contents of the blob object file from the .git/objects directory

    // TODO: Support shortest-unique object hashes
    let file_path = format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]);
    let file = fs::File::open(file_path)?;

    // 2. Decompress the contents using Zlib
    let zlib_decoder = ZlibDecoder::new(file);
    let mut buf_reader = BufReader::new(zlib_decoder);
    let mut buf = Vec::new();
    buf_reader.read_until(0, &mut buf)?;

    // 3. Extract the actual "content" from the decompressed data
    let header = CStr::from_bytes_with_nul(&buf).expect("should be exactly one nul");
    let header = header.to_str()?;

    let Some((kind, size)) = header.split_once(' ') else {
        return Err(GitError::InvalidHeader);
    };

    if kind != "blob" {
        return Err(GitError::InvalidObjectKind(kind.into()));
    }

    // 4. Print the content to stdout

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let size = size.parse::<usize>()?;
    let mut limit_reader = LimitReader::new(buf_reader, size);

    let bytes_read = std::io::copy(&mut limit_reader, &mut stdout)? as usize;
    if bytes_read != size {
        return Err(GitError::InvalidContentSize {
            expected: size,
            actual: bytes_read,
        });
    }

    Ok(())
}

pub fn hash_object(file_path: PathBuf, write: bool) -> Result<(), GitError> {
    fn write_blob<W: Write>(file_path: PathBuf, writer: W) -> Result<String, GitError> {
        let stat = std::fs::metadata(&file_path)?;

        let zlib_encoder = ZlibEncoder::new(writer, Compression::default());
        let mut hash_writer = HashWriter::new(zlib_encoder, Sha1::new());

        write!(hash_writer, "blob ")?;
        write!(hash_writer, "{}\0", stat.len())?;

        let mut file = fs::File::open(file_path)?;
        std::io::copy(&mut file, &mut hash_writer)?;

        let _ = hash_writer.writer.finish()?;
        let hash = hash_writer.hasher.finalize();

        Ok(hex::encode(hash))
    }

    let hash = if write {
        let tmp = "temp";
        let temp_file = std::fs::File::create(tmp)?;
        let hash = write_blob(file_path, temp_file)?;
        fs::create_dir_all(format!(".git/objects/{}/", &hash[..2]))?;

        let file_name = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
        fs::rename(tmp, file_name)?;
        hash
    } else {
        write_blob(file_path, std::io::sink())?
    };

    println!("{}", hash);

    Ok(())
}
