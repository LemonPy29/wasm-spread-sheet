use crate::{buffer::Numeric, ParsedBytes};

use lazy_static::lazy_static;
use lexical::{parse, FromLexical};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

#[repr(usize)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Codes {
    Null = 0,
    Boolean = 1,
    Int32 = 2,
    Int64 = 3,
    Int128 = 4,
    Float32 = 5,
    Float64 = 6,
    Any = 7,
    TmpInt = 99,
    TmpFloat = 100,
}

#[derive(Debug, PartialEq)]
pub enum StageOne<'a> {
    Int(&'a str),
    Float(&'a str),
    Boolean(&'a str),
    Any(&'a str),
}

impl<'a> From<StageOne<'a>> for Codes {
    fn from(general_type: StageOne) -> Codes {
        match general_type {
            StageOne::Float(_) => Codes::TmpFloat,
            StageOne::Int(_) => Codes::TmpInt,
            StageOne::Boolean(_) => Codes::Boolean,
            StageOne::Any(_) => Codes::Any,
        }
    }
}

pub enum IntegerTypes {
    Int32(i32),
    Int64(i64),
    Int128(i128),
}

impl From<IntegerTypes> for Codes {
    fn from(itype: IntegerTypes) -> Codes {
        match itype {
            IntegerTypes::Int32(_) => Codes::Int32,
            IntegerTypes::Int64(_) => Codes::Int64,
            IntegerTypes::Int128(_) => Codes::Int128,
        }
    }
}

impl From<&str> for IntegerTypes {
    fn from(cell: &str) -> IntegerTypes {
        cell.parse::<i32>()
            .map(IntegerTypes::Int32)
            .or_else(|_| cell.parse::<i64>().map(IntegerTypes::Int64))
            .or_else(|_| cell.parse::<i128>().map(IntegerTypes::Int128))
            .expect("Integer overflow")
    }
}

pub enum FloatTypes {
    Float32(f32),
    Float64(f64),
}

impl From<FloatTypes> for Codes {
    fn from(ftype: FloatTypes) -> Codes {
        match ftype {
            FloatTypes::Float32(_) => Codes::Float32,
            FloatTypes::Float64(_) => Codes::Float64,
        }
    }
}

impl From<&str> for FloatTypes {
    fn from(cell: &str) -> FloatTypes {
        cell.parse::<f32>()
            .map(FloatTypes::Float32)
            .or_else(|_| cell.parse::<f64>().map(FloatTypes::Float64))
            .expect("Float overflow")
    }
}

lazy_static! {
    static ref FLOAT: Regex = Regex::new(r"^\s*-?(\d*\.\d+)$").unwrap();
    static ref INTEGER: Regex = Regex::new(r"^\s*-?(\d+)$").unwrap();
    static ref BOOL: Regex = RegexBuilder::new(r"^\s*(true)$|^(false)$")
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[allow(clippy::needless_lifetimes)]
pub fn first_phase<'a>(word: &'a str) -> StageOne {
    if FLOAT.is_match(word) {
        StageOne::Float(word)
    } else if INTEGER.is_match(word) {
        StageOne::Int(word)
    } else if BOOL.is_match(word) {
        StageOne::Boolean(word)
    } else {
        StageOne::Any(word)
    }
}

pub fn bytes_to_bool(bytes: &[u8]) -> Option<bool> {
    if bytes.eq_ignore_ascii_case(b"true") || bytes.eq_ignore_ascii_case(b"\"true\"") {
        Some(true)
    } else if bytes.eq_ignore_ascii_case(b"false") || bytes.eq_ignore_ascii_case(b"\"false\"") {
        Some(false)
    } else {
        None
    }
}

pub fn parse_type<T: Numeric + FromLexical>(words: ParsedBytes) -> Vec<Option<T>> {
    let mut ret = Vec::new();
    words.into_iter().for_each(|bytes| {
        let el = parse(bytes).ok();
        ret.push(el);
    });
    ret
}

pub fn parse_bool(words: ParsedBytes) -> Vec<Option<bool>> {
    let mut ret = Vec::new();
    words.into_iter().for_each(|bytes| {
        let el = bytes_to_bool(bytes);
        ret.push(el);
    });
    ret
}

pub fn parse_utf8(words: ParsedBytes) -> Vec<Option<String>> {
    let mut ret = Vec::new();
    words.into_iter().for_each(|bytes| {
        let el = String::from_utf8(bytes.into()).ok();
        ret.push(el);
    });
    ret
}
