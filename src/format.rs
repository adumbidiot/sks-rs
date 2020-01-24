/// Utilities for working with the as3 file format
pub mod as3;
///Utilities for working with the lbl file format
pub mod lbl;

use crate::block::Block;

/// The file formats this library can parse
#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    LBL,
    AS3,
}

/// Try to guess the file format from a string
pub fn guess_format(data: &str) -> Option<FileFormat> {
    let mut iter = data.trim().lines();
    let first = iter.next()?;

    if Block::from_lbl(first).is_ok() {
        return Some(FileFormat::LBL);
    }

    if first.starts_with("lvlArray") {
        return Some(FileFormat::AS3);
    }

    None
}

/// Try to decode a file of unknown type
pub fn decode(data: &str) -> Result<Vec<Block>, DecodeError> {
    let fmt = guess_format(data).ok_or(DecodeError::UnknownFileFormat)?;
    match fmt {
        FileFormat::LBL => crate::format::lbl::decode(data).map_err(DecodeError::Lbl),
        FileFormat::AS3 => crate::format::as3::decode(data).map_err(DecodeError::As3),
    }
}

/// Errors that can occur while decoding a file of unknown type
#[derive(Debug)]
pub enum DecodeError {
    UnknownFileFormat,
    Lbl(crate::format::lbl::Error),
    As3(crate::format::as3::Error),
}
