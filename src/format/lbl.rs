use crate::Block;

/// Parse an lbl file. This is a compact, yet readable level representation. It is the core of block representation. Look at the tests for an example file.
pub fn decode(data: &str) -> Result<Vec<Block>, DecodeError> {
    let ret = data
        .lines()
        .map(|s| Block::from_lbl(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|s| DecodeError::UnknownLbl(s.into()))?;

    let len = ret.len();
    if len != crate::LEVEL_SIZE {
        return Err(DecodeError::InvalidLength(len));
    }

    Ok(ret)
}

/// Errors that can occur while parsing an lbl file
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// Invalid lbl
    #[error("unknown lbl '{0}'")]
    UnknownLbl(String),

    /// Invalid level length
    #[error("invalid length '{0}'")]
    InvalidLength(usize),
}

/// Encode a level as lbl
pub fn encode(blocks: &[Block]) -> Result<String, EncodeError> {
    let len = blocks.len();
    if len != crate::LEVEL_SIZE {
        return Err(EncodeError::InvalidLength(len));
    }

    let mut ret = String::with_capacity(len * 3); // Conservative estimate: 2 for lbl + 1 for '\n'
    for block in blocks {
        ret += &block.as_lbl();
        ret += "\n";
    }

    Ok(ret)
}

/// Errors that can occur while encoding a level as lbl
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error("invalid length '{0}'")]
    InvalidLength(usize),
}
