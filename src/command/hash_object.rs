use crate::{error::GitError, hash_writer::HashWriter};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::PathBuf};

pub fn run(file_path: PathBuf, write: bool) -> Result<(), GitError> {
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
