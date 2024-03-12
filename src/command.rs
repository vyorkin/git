use std::{
    ffi::CStr,
    fs,
    io::{BufRead, BufReader, Read, Write},
};

use flate2::read::ZlibDecoder;

use crate::error::GitError;

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

    let size = size.parse::<usize>()?;

    buf.clear();
    buf.resize(size, 0);

    buf_reader.read_exact(&mut buf)?;

    let bytes_read = buf_reader.read(&mut [0])?;
    if bytes_read != 0 {
        return Err(GitError::InvalidContentSize);
    }

    // 4. Print the content to stdout
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(&buf)?;

    Ok(())
}
