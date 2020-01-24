use crate::block::Block;

/// Parse an lbl file. This is a compact, yet readable level representation. It is the core of block representation. Look at the tests for an example file.
pub fn decode(data: &str) -> Result<Vec<Block>, Error> {
    let ret = data
        .lines()
        .map(|s| Block::from_lbl(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|s| Error::UnknownLbl(s.into()))?;

    let len = ret.len();
    if len != crate::LEVEL_SIZE {
        return Err(Error::InvalidLength(len));
    }

    Ok(ret)
}

/// Errors that can occur while parsing an lbl file
#[derive(Debug)]
pub enum Error {
    UnknownLbl(String),
    InvalidLength(usize),
}
