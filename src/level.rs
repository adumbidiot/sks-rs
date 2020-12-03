use crate::{
    block::BackgroundType,
    Block,
    LEVEL_SIZE,
};

/// A Game Level
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Level {
    level_data: Vec<Block>,
    is_dark: bool,
    background: BackgroundType,
}

impl Level {
    /// Create a new empty game level
    pub fn new() -> Self {
        Level {
            level_data: vec![Block::Empty; LEVEL_SIZE],
            is_dark: false,
            background: BackgroundType::Cobble,
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

    /// Tries to import a level from a block array
    pub fn from_block_array(blocks: &[Block]) -> Option<Self> {
        if blocks.len() != LEVEL_SIZE {
            return None;
        }

        let mut level = Level::new();

        for (level_block, block) in level.level_data.iter_mut().zip(blocks.iter()) {
            let block = match block {
                Block::Background { background_type } => {
                    level.background = background_type.clone();
                    Block::Empty
                }
                Block::Dark => {
                    level.is_dark = true;
                    Block::Empty
                }
                b => b.clone(),
            };

            *level_block = block;
        }

        Some(level)
    }

    /// Tries to export a block array
    pub fn export_block_array(&self) -> Option<Vec<Block>> {
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
            Some(data)
        } else {
            None
        }
    }

    /// Sets whether the level is dark
    pub fn set_dark(&mut self, is_dark: bool) {
        self.is_dark = is_dark;
    }

    /// Checks whether the level is dark
    pub fn is_dark(&self) -> bool {
        self.is_dark
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
