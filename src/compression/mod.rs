// - Parent
use super::*;

// - STD
use std::io::{Read, Seek, SeekFrom};

// - modules
mod constants;

// - re-exports
use constants::*;

// - external
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use lz4_flex::frame::FrameDecoder as Lz4FrameDecoder;
use xz2::read::XzDecoder;
use zstd::stream::read::Decoder as ZstdDecoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Gunzip,
    Bzip2,
    Lz4,
    Zstd,
    Xz,
}

/// Will try to detect the appropriate compression algorithm by using [detect_compression]
/// and returns a reader which will transparently decompress the data while reading.
/// Returns a normal reader if no compression was detected.
pub fn decompress<I: Read + Seek + 'static>(mut input: I) -> Result<Box<dyn Read>> {
    match detect_compression(&mut input)? {
        CompressionAlgorithm::None => Ok(Box::new(input)),
        CompressionAlgorithm::Gunzip => Ok(Box::new(GzDecoder::new(input))),
        CompressionAlgorithm::Bzip2 => Ok(Box::new(BzDecoder::new(input))),
        CompressionAlgorithm::Lz4 => Ok(Box::new(Lz4FrameDecoder::new(input))),
        CompressionAlgorithm::Zstd => Ok(Box::new(ZstdDecoder::new(input)?)),
        CompressionAlgorithm::Xz => Ok(Box::new(XzDecoder::new(input))),
    }
}

/// Tries to detect the used compression algorithm by checking the first appropriate
/// bytes (and compare them with the magic bytes of the corresponding compression type).)
pub fn detect_compression<I: Read + Seek>(input: &mut I) -> Result<CompressionAlgorithm> {

    let seek_position = input.stream_position()?;
    let mut buffer = [0u8; 6];
    input.read(&mut buffer)?;

    input.seek(SeekFrom::Start(seek_position))?;
    
    if buffer.starts_with(&MAGIC_BYTES_GZIP) {
        #[cfg(feature = "log")]
        trace!("Gz compression detected.");
        Ok(CompressionAlgorithm::Gunzip)
    } else if buffer.starts_with(&MAGIC_BYTES_BZIP2) {
        #[cfg(feature = "log")]
        trace!("Bz2 compression detected.");
        Ok(CompressionAlgorithm::Bzip2)
    } else if buffer.starts_with(&MAGIC_BYTES_LZ4) {
        #[cfg(feature = "log")]
        trace!("Lz4 compression detected.");
        Ok(CompressionAlgorithm::Lz4)
    } else if buffer.starts_with(&MAGIC_BYTES_ZSTD) {
        #[cfg(feature = "log")]
        trace!("Zstd compression detected.");
        Ok(CompressionAlgorithm::Zstd)
    } else if buffer.starts_with(&MAGIC_BYTES_XZ) {
        #[cfg(feature = "log")]
        trace!("Xz compression detected.");
        Ok(CompressionAlgorithm::Xz)
    } else {
        #[cfg(feature = "log")]
        trace!("No compression detected.");
        Ok(CompressionAlgorithm::None)
    }
}

