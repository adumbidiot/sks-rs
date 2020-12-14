/// Utilities related to how this lib represents blocks
pub mod block;
/// Utilities for working with file formats
pub mod format;
/// Utilities for level representation
pub mod level;
/// Utilities for rendering blocks
pub mod render;

/// The width of a level, in blocks
pub const LEVEL_WIDTH: usize = 32;
/// The height of a level, in blocks
pub const LEVEL_HEIGHT: usize = 18;
/// The length of a level, in blocks. Equal to width * height.
pub const LEVEL_SIZE: usize = LEVEL_WIDTH * LEVEL_HEIGHT;

pub use crate::{
    block::Block,
    level::Level,
};
