use crate::{
    block::BackgroundType,
    format::{
        EncodeError,
        FileFormat,
        LevelNumber,
    },
    Block,
    LEVEL_SIZE,
};
use std::convert::TryInto;

/// Errors that occur while interacting with level
#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    /// Could not encode the following logic blocks
    #[error("extra logic blocks")]
    ExtraLogicBlocks(Vec<Block>),

    #[error("{0}")]
    Encode(#[from] EncodeError),
}

/// A Game Level
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Level {
    /// Internal level data. Does not contain logic types.
    level_data: Vec<Block>,

    /// Whether the level is dark
    is_dark: bool,

    /// Background type
    background: BackgroundType,

    /// Level Number
    level_number: Option<LevelNumber>,
}

impl Level {
    /// Create a new empty game level
    pub fn new() -> Self {
        Level {
            level_data: vec![Block::Empty; LEVEL_SIZE],
            is_dark: false,
            background: BackgroundType::Cobble,

            level_number: None,
        }
    }

    /// Get internal level data. This will not contain logic types like backgrounds and dark blocks
    pub fn get_level_data(&self) -> &[Block] {
        &self.level_data
    }

    /// Try to insert a block at the index. Returns the block if it fails.
    pub fn add_block(&mut self, i: usize, block: Block) -> Option<Block> {
        if let Some(level_block) = self.level_data.get_mut(i) {
            *level_block = block;
            None
        } else {
            Some(block)
        }
    }

    /// Tries to create a level from a block array
    pub fn from_block_array(blocks: &[Block; LEVEL_SIZE]) -> Self {
        let mut level = Level::new();
        level.import_block_array(blocks);
        level
    }

    /// Tries to import a level from a block array
    pub fn import_block_array(&mut self, blocks: &[Block; LEVEL_SIZE]) {
        self.background = BackgroundType::Cobble;
        self.is_dark = false;

        for (level_block, block) in self.level_data.iter_mut().zip(blocks.iter()) {
            let block = match block {
                Block::Background { background_type } => {
                    self.background = background_type.clone();
                    Block::Empty
                }
                Block::Dark => {
                    self.is_dark = true;
                    Block::Empty
                }
                b => b.clone(),
            };

            *level_block = block;
        }
    }

    /// Guess format from string and try to import it
    pub fn import_str(&mut self, data: &str) -> Result<(), crate::format::DecodeError> {
        let (level_number, blocks) = crate::format::decode(data)?;
        self.import_block_array(
            blocks
                .as_slice()
                .try_into()
                .expect("Vec is sized correctly"),
        );
        self.level_number = level_number;
        Ok(())
    }

    /// Tries to export a block array
    pub fn export_block_array(&self) -> Result<Vec<Block>, LevelError> {
        let mut to_insert = Vec::with_capacity(2);
        if self.is_dark() {
            to_insert.push(Block::Dark);
        }

        if self.background != BackgroundType::Cobble {
            to_insert.push(Block::Background {
                background_type: self.background.clone(),
            });
        }

        let data = self
            .get_level_data()
            .iter()
            .map(|block| {
                if block.is_empty() {
                    if let Some(block) = to_insert.pop() {
                        return block;
                    }
                }
                block.clone()
            })
            .collect();

        if to_insert.is_empty() {
            Ok(data)
        } else {
            Err(LevelError::ExtraLogicBlocks(to_insert))
        }
    }

    /// Try to export the level as a string with the given format
    pub fn export_str(&self, format: FileFormat) -> Result<String, LevelError> {
        Ok(crate::format::encode(
            &self.export_block_array()?,
            &format,
            self.level_number.as_ref(),
        )?)
    }

    /// Sets whether the level is dark
    pub fn set_dark(&mut self, is_dark: bool) {
        self.is_dark = is_dark;
    }

    /// Checks whether the level is dark
    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    /// Get the level number
    pub fn get_level_number(&self) -> Option<&LevelNumber> {
        self.level_number.as_ref()
    }

    /// Set the level number
    pub fn set_level_number(&mut self, level_number: Option<LevelNumber>) {
        self.level_number = level_number;
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
