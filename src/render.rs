use crate::block::{
    BackgroundType,
    Block,
    Direction,
};
use std::collections::HashMap;

macro_rules! load_blocks {
    (
        $(
            $b:ident
        ),*
    ) => {
        $(
            pub const $b: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", stringify!($b), ".png"));
        )*
    }
}

load_blocks! {
    M0,

    B0,
    E0,
    IK,
    BK,
    NO,
    OD,
    OU,
    OL,
    OR,
    CI,
    CO,
    CP,
    CS,
    X0,
    P0,
    P1,
    D0,
    S0,
    T0,
    T1,
    D1
}

/// A block renderer based on the image crate
pub struct ImageRenderer {
    cache: HashMap<ImageRequest, Option<image::DynamicImage>>,
}

impl ImageRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Render a level. blocks must be the right size.
    pub fn render(
        &mut self,
        blocks: &[Block],
        options: &RenderOptions,
    ) -> Result<image::DynamicImage, RenderError> {
        let len = blocks.len();
        if len != crate::LEVEL_SIZE {
            return Err(RenderError::InvalidLength(len));
        }

        let mut bg = Block::Background {
            background_type: BackgroundType::Cobble,
        };
        // let dark = false; // TODO: "Dark" Level rendering through render options

        for block in blocks {
            if let Block::Background { .. } = block {
                bg = block.clone();
            }
        }

        let req = ImageRequest {
            w: options.width as u32,
            h: options.height as u32,
            block: bg,
        };

        let mut base = self
            .get_rendered(req)
            .ok_or(RenderError::MissingBackgroundTexture)?
            .clone();

        let w = options.width as u32 / crate::LEVEL_WIDTH as u32;
        let h = options.height as u32 / crate::LEVEL_HEIGHT as u32;

        for (y, row) in blocks.chunks(crate::LEVEL_WIDTH).enumerate() {
            for (x, block) in row.iter().enumerate() {
                if !block.is_background() {
                    let r = ImageRequest {
                        w,
                        h,
                        block: block.clone(),
                    };
                    if let Some(img) = self.get_rendered(r) {
                        image::imageops::overlay(&mut base, img, w * x as u32, h * y as u32);
                    }
                }
            }
        }

        Ok(base)
    }

    /// Get a resized image from the cache, else resize it and cache it, returning a reference.
    pub fn get_rendered(&mut self, r: ImageRequest) -> Option<&image::DynamicImage> {
        self.cache
            .entry(r.clone())
            .or_insert_with(|| Self::generate_block_image(&r))
            .as_ref()
    }

    /// Generates a new block image. returns None if it is an empty image. TODO: Consider Error Texture
    pub fn generate_block_image(r: &ImageRequest) -> Option<image::DynamicImage> {
        let img = match &r.block {
            Block::Background {
                background_type: BackgroundType::Cobble,
            } => M0,
            Block::Background { .. } => {
                return None;
            }
            Block::Block => B0,
            Block::Dark | Block::Empty => {
                return None;
            }
            Block::Exit => E0,
            Block::Key => IK,
            Block::Lock => BK,
            Block::Note { .. } => NO,
            Block::OneWayWall {
                direction: Direction::Down,
            } => OD,
            Block::OneWayWall {
                direction: Direction::Up,
            } => OU,
            Block::OneWayWall {
                direction: Direction::Left,
            } => OL,
            Block::OneWayWall {
                direction: Direction::Right,
            } => OR,
            Block::PipeIn => CI,
            Block::PipeOut => CO,
            Block::PipePhase => CP,
            Block::PipeSolid => CS,
            Block::Player => X0,
            Block::PowerUpBurrow => P0,
            Block::PowerUpRecall => P1,
            Block::SecretExit => return None,
            Block::Scaffold => D0,
            Block::Switch => S0,
            Block::SwitchCeiling => {
                return None;
            }
            Block::ToggleBlock { solid: true } => T0,
            Block::ToggleBlock { solid: false } => T1,
            Block::Torch => D1,
            Block::Wire => {
                return None;
            }
        };

        Some(
            image::load_from_memory(img)
                .expect("Valid Embedded image")
                .resize(r.w, r.h, image::imageops::FilterType::Triangle),
        )
    }
}

impl Default for ImageRenderer {
    fn default() -> Self {
        Self::new()
    }
}

///Errors that may occur while rendering
#[derive(Debug)]
pub enum RenderError {
    InvalidLength(usize),
    MissingBackgroundTexture,
}

/// A "request" for block data from the cache
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImageRequest {
    pub w: u32,
    pub h: u32,
    pub block: Block,
}

/// Options for rendering
#[derive(Debug)]
pub struct RenderOptions {
    pub width: usize,
    pub height: usize,
}

impl RenderOptions {
    /// Default RenderOptions.
    pub fn new() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }

    /// Requested Width. Note: if not 16:9, stretching will occur
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Requested Height. Note: if not 16:9, stretching will occur
    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self::new()
    }
}
