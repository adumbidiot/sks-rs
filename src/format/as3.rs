use crate::block::Block;
use boa::syntax::{
    ast::{
        constant::Const,
        node::Node,
    },
    parser::error::ParseError,
};

/// Try to decode a string as an as3 file format. View the tests to see what a valid file of this kind looks like.
pub fn decode(data: &str) -> Result<(LevelNum, Vec<Block>), DecodeError> {
    let statement_list = boa::parse(data)?;

    let mut height = 0;
    let mut level_num: Option<LevelNum> = None;
    let mut ret = Vec::with_capacity(crate::LEVEL_SIZE);

    for (i, assign) in statement_list
        .statements()
        .iter()
        .filter_map(|node| match node {
            Node::Assign(assign) => Some(assign),
            _ => None,
        })
        .enumerate()
    {
        let lhs = assign.lhs();
        let rhs = assign.rhs();

        height += 1;
        let new_level_num = parse_lhs(&lhs, i)?;
        match level_num.as_ref() {
            Some(v) => {
                if *v != new_level_num {
                    return Err(DecodeError::InvalidLevelNum {
                        expected: v.clone(),
                        actual: new_level_num,
                    });
                }
            }
            None => {
                level_num = Some(new_level_num);
            }
        }

        ret.extend(parse_row(rhs)?);
    }

    if height != crate::LEVEL_HEIGHT {
        return Err(DecodeError::InvalidHeight(height));
    }

    let size = ret.len();

    if size != crate::LEVEL_SIZE {
        return Err(DecodeError::InvalidLevelSize(size));
    }

    Ok((level_num.ok_or(DecodeError::MissingLevelNum)?, ret))
}

fn parse_lhs(node: &Node, expected_row: usize) -> Result<LevelNum, DecodeError> {
    match node {
        Node::GetField(get_field) => {
            validate_lhs_row_num(get_field.field(), expected_row)?;
            match get_field.obj() {
                Node::GetField(get_field) => {
                    validate_level_array_name(get_field.obj())?;
                    parse_level_num(get_field.field())
                }
                _ => Err(DecodeError::InvalidLhsExpr(node.clone())),
            }
        }
        _ => Err(DecodeError::InvalidGlobalLhsExpr(node.clone())),
    }
}

fn parse_level_num(node: &Node) -> Result<LevelNum, DecodeError> {
    match &node {
        Node::Const(Const::Num(n)) => Ok(LevelNum::Num(*n as usize)),
        Node::Const(Const::Int(n)) => Ok(LevelNum::Num(*n as usize)),
        Node::Const(Const::String(s)) => Ok(LevelNum::String(s.to_string())),
        Node::Identifier(identifier) => Ok(LevelNum::String(identifier.as_ref().to_string())),
        _ => Err(DecodeError::InvalidLevelNumExpr(node.clone())),
    }
}

fn validate_level_array_name(node: &Node) -> Result<(), DecodeError> {
    match &node {
        Node::Identifier(identifier) => {
            if identifier.as_ref() != "lvlArray" {
                return Err(DecodeError::InvalidLevelArrayName(
                    identifier.as_ref().into(),
                ));
            }
            Ok(())
        }
        _ => Err(DecodeError::InvalidLevelArrayNameExpr(node.clone())),
    }
}

fn validate_lhs_row_num(node: &Node, expected_row: usize) -> Result<(), DecodeError> {
    match &node {
        Node::Const(Const::Num(n)) => {
            let n = *n as usize;
            if n != expected_row {
                return Err(DecodeError::InvalidRowNum {
                    expected: expected_row,
                    actual: n,
                });
            }
        }
        Node::Const(Const::Int(n)) => {
            let n = *n as usize;
            if n != expected_row {
                return Err(DecodeError::InvalidRowNum {
                    expected: expected_row,
                    actual: n,
                });
            }
        }
        _ => {
            return Err(DecodeError::InvalidRowNumExpr(node.clone()));
        }
    }

    Ok(())
}

fn parse_row(node: &Node) -> Result<Vec<Block>, DecodeError> {
    match &node {
        Node::ArrayDecl(exprs) => {
            let width = exprs.as_ref().len();
            if width != crate::LEVEL_WIDTH {
                return Err(DecodeError::InvalidWidth(width));
            }

            exprs.as_ref().iter().map(parse_cell).collect()
        }
        _ => Err(DecodeError::InvalidRowExpr(node.clone())),
    }
}

fn parse_cell(node: &Node) -> Result<Block, DecodeError> {
    match &node {
        Node::Const(Const::Num(n)) => {
            if *n == 0.0 {
                Ok(Block::Empty)
            } else {
                Err(DecodeError::InvalidLbl(n.to_string()))
            }
        }
        Node::Const(Const::Int(n)) => {
            if *n == 0 {
                Ok(Block::Empty)
            } else {
                Err(DecodeError::InvalidLbl(n.to_string()))
            }
        }
        Node::Const(Const::String(v)) => {
            Block::from_lbl(v).map_err(|s| DecodeError::InvalidLbl(s.into()))
        }
        Node::Identifier(v) => {
            Block::from_lbl(v.as_ref()).map_err(|s| DecodeError::InvalidLbl(s.into()))
        }
        _ => Err(DecodeError::InvalidCellExpr(node.clone())),
    }
}

/// The level this file advertisies itself to be. While usually a number, like 0, It CAN be a literal, like: X. If a float is provided, it is casted to an int through truncating.
#[derive(Debug, Clone, PartialEq)]
pub enum LevelNum {
    String(String),
    Num(usize),
}

impl std::fmt::Display for LevelNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Num(n) => n.fmt(f),
        }
    }
}

/// The errors reading an as3 file can have.
#[derive(Debug)]
pub enum DecodeError {
    Parser(ParseError),

    InvalidHeight(usize),

    InvalidGlobalLhsExpr(Node),
    InvalidLhsExpr(Node),

    InvalidLevelArrayNameExpr(Node),
    InvalidLevelArrayName(String),

    InvalidLevelNumExpr(Node),
    InvalidLevelNum {
        expected: LevelNum,
        actual: LevelNum,
    },

    InvalidRowNumExpr(Node),
    InvalidRowNum {
        expected: usize,
        actual: usize,
    },

    InvalidRowExpr(Node),
    InvalidWidth(usize),

    InvalidCellExpr(Node),
    InvalidLbl(String),

    InvalidLevelSize(usize),

    MissingLevelNum,
}

impl From<ParseError> for DecodeError {
    fn from(e: ParseError) -> Self {
        DecodeError::Parser(e)
    }
}

/// Encode blocks to as3
pub fn encode(blocks: &[Block], level_num: &LevelNum) -> Result<String, EncodeError> {
    let len = blocks.len();
    if len != crate::LEVEL_SIZE {
        return Err(EncodeError::InvalidLength(len));
    }

    let mut ret = String::new(); //TODO: Find good size to preallocate

    for (i, row) in blocks.chunks(crate::LEVEL_WIDTH).enumerate() {
        ret += &format!("lvlArray[{}][{}] = [", level_num, i);
        for (j, block) in row.iter().enumerate() {
            match block {
                Block::Note { .. } => {
                    ret += "\"";
                    ret += &block.as_lbl();
                    ret += "\"";
                }
                _ => {
                    ret += &block.as_lbl();
                }
            }

            if j == crate::LEVEL_WIDTH - 1 {
                ret += "];\n"
            } else {
                ret += ", ";
            }
        }
    }

    Ok(ret)
}

/// Errors that can occur while encoding as3
#[derive(Debug)]
pub enum EncodeError {
    InvalidLength(usize),
}
