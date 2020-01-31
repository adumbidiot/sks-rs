use std::borrow::Cow;

/// An entity that occupies a space in lbl representation.
/// Also the internal rep  of a "block" in this library.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    Background { background_type: BackgroundType },
    Block,
    Dark,
    Empty,
    Exit,
    Key,
    Lock,
    Note { text: String },
    Scaffold,
    SecretExit,
    Switch,
    SwitchCeiling,
    OneWayWall { direction: Direction },
    PipeIn,
    PipeOut,
    PipePhase,
    PipeSolid,
    Player,
    PowerUpBurrow,
    PowerUpRecall,
    ToggleBlock { solid: bool },
    Torch,
    Wire,
}

impl Block {
    /// Decodes an LBL string to a block, if valid
    pub fn from_lbl(data: &str) -> Result<Block, &str> {
        match data {
            "00" => Ok(Block::Empty),
            "A0" => Ok(Block::Dark),
            "B0" => Ok(Block::Block),
            "BK" => Ok(Block::Lock),
            "CI" => Ok(Block::PipeIn),
            "CO" => Ok(Block::PipeOut),
            "CP" => Ok(Block::PipePhase),
            "CS" => Ok(Block::PipeSolid),
            "D0" => Ok(Block::Scaffold),
            "D1" => Ok(Block::Torch),
            "E0" => Ok(Block::Exit),
            "E1" => Ok(Block::SecretExit),
            "IK" => Ok(Block::Key),
            "M0" => Ok(Block::Background {
                background_type: BackgroundType::Cobble,
            }),
            "M1" => Ok(Block::Background {
                background_type: BackgroundType::Waterfall,
            }),
            "M2" => Ok(Block::Background {
                background_type: BackgroundType::Skullfall,
            }),
            "M3" => Ok(Block::Background {
                background_type: BackgroundType::Concrete,
            }),
            "M4" => Ok(Block::Background {
                background_type: BackgroundType::Reserved1,
            }),
            "M5" => Ok(Block::Background {
                background_type: BackgroundType::Reserved2,
            }),
            "M6" => Ok(Block::Background {
                background_type: BackgroundType::Reserved3,
            }),
            "OD" => Ok(Block::OneWayWall {
                direction: Direction::Down,
            }),
            "OL" => Ok(Block::OneWayWall {
                direction: Direction::Left,
            }),
            "OR" => Ok(Block::OneWayWall {
                direction: Direction::Right,
            }),
            "OU" => Ok(Block::OneWayWall {
                direction: Direction::Up,
            }),
            "P0" => Ok(Block::PowerUpBurrow),
            "P1" => Ok(Block::PowerUpRecall),
            "S0" => Ok(Block::Switch),
            "S1" => Ok(Block::SwitchCeiling),
            "T0" => Ok(Block::ToggleBlock { solid: true }),
            "T1" => Ok(Block::ToggleBlock { solid: false }),
            "X0" => Ok(Block::Player),
            "WR" => Ok(Block::Wire),
            data => {
                let note_prefix = "Note:";
                if data.starts_with(note_prefix) {
                    let text = String::from(data.get(note_prefix.len()..).unwrap());
                    Ok(Block::Note { text })
                } else {
                    Err(data)
                }
            }
        }
    }

    /// Produces the LBL encoding of a given block
    pub fn as_lbl(&self) -> Cow<'static, str> {
        match self {
            Block::Background {
                background_type: BackgroundType::Cobble,
            } => "M0".into(),
            Block::Background {
                background_type: BackgroundType::Waterfall,
            } => "M1".into(),
            Block::Background {
                background_type: BackgroundType::Skullfall,
            } => "M2".into(),
            Block::Background {
                background_type: BackgroundType::Concrete,
            } => "M3".into(),
            Block::Background {
                background_type: BackgroundType::Reserved1,
            } => "M4".into(),
            Block::Background {
                background_type: BackgroundType::Reserved2,
            } => "M5".into(),
            Block::Background {
                background_type: BackgroundType::Reserved3,
            } => "M6".into(),
            Block::Block => "B0".into(),
            Block::Dark => "A0".into(),
            Block::Empty => "00".into(),
            Block::Exit => "E0".into(),
            Block::Key => "IK".into(),
            Block::Lock => "BK".into(),
            Block::Note { text } => format!("Note:{}", text).into(),
            Block::OneWayWall {
                direction: Direction::Down,
            } => "OD".into(),
            Block::OneWayWall {
                direction: Direction::Up,
            } => "OU".into(),
            Block::OneWayWall {
                direction: Direction::Left,
            } => "OL".into(),
            Block::OneWayWall {
                direction: Direction::Right,
            } => "OR".into(),
            Block::PipeIn => "CI".into(),
            Block::PipeOut => "CO".into(),
            Block::PipePhase => "CP".into(),
            Block::PipeSolid => "CS".into(),
            Block::Player => "X0".into(),
            Block::PowerUpBurrow => "P0".into(),
            Block::PowerUpRecall => "P1".into(),
            Block::SecretExit => "E1".into(),
            Block::Scaffold => "D0".into(),
            Block::Switch => "S0".into(),
            Block::SwitchCeiling => "S1".into(),
            Block::ToggleBlock { solid: true } => "T0".into(),
            Block::ToggleBlock { solid: false } => "T1".into(),
            Block::Torch => "D1".into(),
            Block::Wire => "WR".into(),
        }
    }

    pub fn is_background(&self) -> bool {
        match self {
            Self::Background { .. } => true,
            _ => false,
        }
    }
}

impl Default for Block {
    /// An empty block
    fn default() -> Self {
        Block::Empty
    }
}

/// The directions something could face
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The types for backgrounds
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackgroundType {
    Cobble,
    Waterfall,
    Skullfall,
    Concrete,
    Reserved1,
    Reserved2,
    Reserved3,
}
