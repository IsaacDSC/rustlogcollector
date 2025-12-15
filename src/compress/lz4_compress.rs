use std::io::Error;

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

pub type CompressedData = Vec<u8>;

pub fn compress(data: &[u8]) -> CompressedData {
    compress_prepend_size(data)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, Error> {
    match lz4_flex::decompress_size_prepended(data) {
        Ok(decompressed_data) => Ok(decompressed_data),
        Err(err) => Err(Error::new(std::io::ErrorKind::Other, err)),
    }
}
