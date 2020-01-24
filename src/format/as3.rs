use crate::block::Block;
use boa::syntax::{
    ast::{
        constant::Const,
        expr::{
            Expr,
            ExprDef,
        },
    },
    lexer::{
        Lexer,
        LexerError,
    },
    parser::{
        ParseError,
        Parser,
    },
};

/// Try to decode a string as an as3 file format. View the tests to see what a valid file of this kind looks like.
pub fn decode(data: &str) -> Result<(LevelNum, Vec<Block>), DecodeError> {
    let mut lexer = Lexer::new(data);
    lexer.lex().map_err(DecodeError::Lexer)?;

    let mut parser = Parser::new(lexer.tokens);
    let expr = parser.parse_all().map_err(DecodeError::Parser)?;

    match &expr.def {
        ExprDef::Block(exprs) => {
            let mut height = 0;
            let mut level_num: Option<LevelNum> = None;
            let mut ret = Vec::with_capacity(crate::LEVEL_SIZE);

            for (i, (lhs, rhs)) in exprs
                .iter()
                .filter_map(|expr| match &expr.def {
                    ExprDef::Assign(lhs, rhs) => Some((lhs, rhs)),
                    _ => None,
                })
                .enumerate()
            {
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
        _ => Err(DecodeError::InvalidBaseExpr(expr)),
    }
}

fn parse_lhs(expr: &Expr, expected_row: usize) -> Result<LevelNum, DecodeError> {
    match &expr.def {
        ExprDef::GetField(lhs, rhs) => {
            validate_lhs_row_num(rhs, expected_row)?;
            match &lhs.def {
                ExprDef::GetField(lhs, rhs) => {
                    validate_level_array_name(lhs)?;
                    parse_level_num(rhs)
                }
                _ => Err(DecodeError::InvalidLhsExpr(expr.clone())),
            }
        }
        _ => Err(DecodeError::InvalidGlobalLhsExpr(expr.clone())),
    }
}

fn parse_level_num(expr: &Expr) -> Result<LevelNum, DecodeError> {
    match &expr.def {
        ExprDef::Const(Const::Num(n)) => Ok(LevelNum::Num(*n as usize)),
        ExprDef::Const(Const::String(s)) => Ok(LevelNum::String(s.clone())),
        ExprDef::Local(s) => Ok(LevelNum::String(s.clone())),
        _ => Err(DecodeError::InvalidLevelNumExpr(expr.clone())),
    }
}

fn validate_level_array_name(expr: &Expr) -> Result<(), DecodeError> {
    match &expr.def {
        ExprDef::Local(s) => {
            if s != "lvlArray" {
                Err(DecodeError::InvalidLevelArrayName(s.into()))
            } else {
                Ok(())
            }
        }
        _ => Err(DecodeError::InvalidLevelArrayNameExpr(expr.clone())),
    }
}

fn validate_lhs_row_num(expr: &Expr, expected_row: usize) -> Result<(), DecodeError> {
    match &expr.def {
        ExprDef::Const(Const::Num(n)) => {
            let n = *n as usize;
            if n != expected_row {
                return Err(DecodeError::InvalidRowNum {
                    expected: expected_row,
                    actual: n,
                });
            }
        }
        _ => {
            return Err(DecodeError::InvalidRowNumExpr(expr.clone()));
        }
    }

    Ok(())
}

fn parse_row(expr: &Expr) -> Result<Vec<Block>, DecodeError> {
    match &expr.def {
        ExprDef::ArrayDecl(exprs) => {
            let width = exprs.len();
            if width != crate::LEVEL_WIDTH {
                return Err(DecodeError::InvalidWidth(width));
            }

            exprs.iter().map(parse_cell).collect()
        }
        _ => Err(DecodeError::InvalidRowExpr(expr.clone())),
    }
}

fn parse_cell(expr: &Expr) -> Result<Block, DecodeError> {
    match &expr.def {
        ExprDef::Const(Const::Num(n)) => {
            if *n == 0.0 {
                Ok(Block::Empty)
            } else {
                Err(DecodeError::InvalidLbl(n.to_string()))
            }
        }
        ExprDef::Const(Const::String(v)) => {
            Block::from_lbl(v).map_err(|s| DecodeError::InvalidLbl(s.into()))
        }
        ExprDef::Local(v) => Block::from_lbl(v).map_err(|s| DecodeError::InvalidLbl(s.into())),
        _ => Err(DecodeError::InvalidCellExpr(expr.clone())),
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
    Lexer(LexerError),
    Parser(ParseError),

    InvalidBaseExpr(Expr),
    InvalidHeight(usize),

    InvalidGlobalLhsExpr(Expr),
    InvalidLhsExpr(Expr),

    InvalidLevelArrayNameExpr(Expr),
    InvalidLevelArrayName(String),

    InvalidLevelNumExpr(Expr),
    InvalidLevelNum {
        expected: LevelNum,
        actual: LevelNum,
    },

    InvalidRowNumExpr(Expr),
    InvalidRowNum {
        expected: usize,
        actual: usize,
    },

    InvalidRowExpr(Expr),
    InvalidWidth(usize),

    InvalidCellExpr(Expr),
    InvalidLbl(String),

    InvalidLevelSize(usize),

    MissingLevelNum,
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
