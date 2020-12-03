/// Utilities for working with the as3 file format
pub mod as3;
///Utilities for working with the lbl file format
pub mod lbl;

use crate::block::Block;

/// The file formats this library can parse
#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Lbl,
    As3,
}

/// Try to guess the file format from a string
pub fn guess_format(data: &str) -> Option<FileFormat> {
    let mut iter = data.trim().lines();
    let first = iter.next()?;

    if Block::from_lbl(first).is_ok() {
        return Some(FileFormat::Lbl);
    }

    if first.starts_with("lvlArray") || first.starts_with("//") {
        return Some(FileFormat::As3);
    }

    None
}

/// Try to decode a file of unknown type
pub fn decode(data: &str) -> Result<(Option<self::as3::LevelNum>, Vec<Block>), DecodeError> {
    let fmt = guess_format(data).ok_or(DecodeError::UnknownFileFormat)?;
    match fmt {
        FileFormat::Lbl => crate::format::lbl::decode(data)
            .map_err(DecodeError::Lbl)
            .map(|el| (None, el)),
        FileFormat::As3 => crate::format::as3::decode(data)
            .map_err(DecodeError::As3)
            .map(|(n, el)| (Some(n), el)),
    }
}

/// Errors that can occur while decoding a file of unknown type
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// Failed to guess file format
    #[error("unknown file format")]
    UnknownFileFormat,

    /// LBL decode error
    #[error("{0}")]
    Lbl(#[from] self::lbl::DecodeError),

    /// As3 decode error
    #[error("{0}")]
    As3(#[from] self::as3::DecodeError),
}

/// Try to decode a file to a format. level_num is not needed for lbl.
pub fn encode(
    blocks: &[Block],
    format: &FileFormat,
    level_num: Option<&self::as3::LevelNum>,
) -> Result<String, EncodeError> {
    match format {
        FileFormat::Lbl => self::lbl::encode(blocks).map_err(EncodeError::Lbl),
        FileFormat::As3 => {
            self::as3::encode(blocks, level_num.ok_or(EncodeError::MissingLevelNum)?)
                .map_err(EncodeError::As3)
        }
    }
}

/// Errors that can occur while encoding files
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error("missing level number")]
    MissingLevelNum,
    
    #[error("{0}")]
    Lbl(#[from] self::lbl::EncodeError),
    
    #[error("{0}")]
    As3(#[from] self::as3::EncodeError),
}
