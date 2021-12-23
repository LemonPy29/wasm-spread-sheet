use crate::{BaseBuffer, EntryData, Writable, BUFFER_SIZE, ENTRY_DATA};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use wasm_bindgen::prelude::*;

#[repr(usize)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
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

pub trait DataType: Copy + Default + std::str::FromStr {}

impl DataType for bool {}
impl DataType for i32 {}
impl DataType for i64 {}
impl DataType for i128 {}
impl DataType for f32 {}
impl DataType for f64 {}

pub fn parse_type<T: DataType>(words: BaseBuffer<&str>) -> BaseBuffer<Option<T>> {
    let mut ret = BaseBuffer::new();
    words.buffer.iter().for_each(|word| {
        let el = word.parse::<T>().ok();
        ret.write(Writable::Single(el));
    });
    ret
}

pub fn parse_utf8(words: BaseBuffer<&str>) -> BaseBuffer<Option<&str>> {
    let mut ret = BaseBuffer::new();
    words.buffer.iter().for_each(|word| {
        let el = word.is_empty().then(|| *word);
        ret.write(Writable::Single(el));
    });
    ret
}

#[derive(Default)]
pub struct ParsedWords<'a> {
    buffers: Vec<BaseBuffer<&'a str>>,
}

impl<'a> ParsedWords<'a> {
    pub fn write_words(&mut self, data: &'a EntryData) {
        self.buffers.iter_mut().enumerate().for_each(|(i, buffer)| {
            let words: Vec<&str> = data.view(i).split(crate::DELIMITER_TOKEN).collect();
            buffer.write(Writable::Arr(&words));
        })
    }

    #[allow(dead_code)]
    fn generate_codes(&self) -> Vec<Codes> {
        const N_WORDS: usize = (BUFFER_SIZE as f32 * 0.1) as usize;

        self.buffers
            .iter()
            .map(|buffer| {
                let code: Codes = buffer
                    .view(0, N_WORDS)
                    .iter()
                    .map(|word| match first_phase(word) {
                        StageOne::Int(text) => IntegerTypes::from(text).into(),
                        StageOne::Float(text) => FloatTypes::from(text).into(),
                        StageOne::Any(text) if text.is_empty() => Codes::Null,
                        val @ StageOne::Boolean(_) | val @ StageOne::Any(_) => val.into(),
                    })
                    .max()
                    .unwrap();
                code
            })
            .collect()
    }

    fn iter_with_code(self) -> impl Iterator<Item = (Codes, BaseBuffer<&'a str>)> {
        let codes = self.generate_codes();
        codes.into_iter().zip(self.buffers.into_iter())
    }
}

trait ColumnTrait {
    fn len(&self) -> usize;
}

impl<T: DataType> ColumnTrait for BaseBuffer<Option<T>> {
    fn len(&self) -> usize {
        self.offset
    }
}
impl ColumnTrait for BaseBuffer<Option<&str>> {
    fn len(&self) -> usize {
        self.offset
    }
}

pub struct Column(Box<dyn ColumnTrait>);

impl Column {
    fn new<T: DataType + 'static>(buffer: BaseBuffer<Option<T>>) -> Self {
        Self(Box::new(buffer))
    }

    fn from_any(buffer: BaseBuffer<Option<&'static str>>) -> Self {
        Self(Box::new(buffer))
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Frame {
    columns: Vec<Column>,
}

#[wasm_bindgen]
impl Frame {
    pub fn new() -> Self {
        let mut parsed = ParsedWords::default();
        unsafe {
            parsed.write_words(&ENTRY_DATA);
        }

        let columns: Vec<Column> = parsed
            .iter_with_code()
            .map(|(code, buffer)| match code {
                Codes::Boolean => Column::new(parse_type::<bool>(buffer)),
                Codes::Int32 => Column::new(parse_type::<i32>(buffer)),
                Codes::Int64 => Column::new(parse_type::<i64>(buffer)),
                Codes::Int128 => Column::new(parse_type::<i128>(buffer)),
                Codes::Float32 => Column::new(parse_type::<f32>(buffer)),
                Codes::Float64 => Column::new(parse_type::<f64>(buffer)),
                Codes::Any => Column::from_any(parse_utf8(buffer)),
                _ => unreachable!(),
            })
            .collect();

        Self { columns }
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }
}
