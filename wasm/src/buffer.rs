use itertools::Itertools;
use js_sys::JsString;
use lexical::{parse, FromLexical};
use num::Num;

use crate::{type_parser::bytes_to_bool, ParsedBytes};

const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";
pub trait Numeric: Copy + Default + std::str::FromStr + Num {}

impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

trait SeriesTrait {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes);
    fn to_js_string(&self, offset: usize, size: usize) -> JsString;
}

trait ColumnTrait: SeriesTrait {
    fn i32(&self) -> Result<&[Option<i32>], &str> {
        Err("Unexpected type")
    }
    fn i64(&self) -> Result<&[Option<i64>], &str> {
        Err("Unexpected type")
    }
    fn i128(&self) -> Result<&[Option<i128>], &str> {
        Err("Unexpected type")
    }
    fn f32(&self) -> Result<&[Option<f32>], &str> {
        Err("Unexpected type")
    }
    fn f64(&self) -> Result<&[Option<f64>], &str> {
        Err("Unexpected type")
    }
    fn bool(&self) -> Result<&[Option<bool>], &str> {
        Err("Unexpected type")
    }
    fn str(&self) -> Result<&[Option<String>], &str> {
        Err("Unexpected type")
    }
}

impl<T: Numeric + FromLexical> SeriesTrait for Vec<Option<T>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes) {
        bytes.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    #[allow(unstable_name_collisions)]
    fn to_js_string(&self, offset: usize, size: usize) -> JsString {
        let s: String = self
            .iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |number| number.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect();
        JsString::from(s)
    }
}

impl SeriesTrait for Vec<Option<String>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes) {
        bytes.into_iter().for_each(|word| {
            let el = String::from_utf8(word.into()).ok();
            self.push(el);
        })
    }

    #[allow(unstable_name_collisions)]
    fn to_js_string(&self, offset: usize, size: usize) -> JsString {
        let s: String = self
            .iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.as_deref().unwrap_or_default())
            .intersperse(DELIMITER_TOKEN)
            .collect();
        JsString::from(s)
    }
}

impl ColumnTrait for Vec<Option<i32>> {
    fn i32(&self) -> Result<&[Option<i32>], &'static str> {
        Ok(&self[..])
    }
}

impl ColumnTrait for Vec<Option<i64>> {
    fn i64(&self) -> Result<&[Option<i64>], &'static str> {
        Ok(&self[..])
    }
}

impl ColumnTrait for Vec<Option<i128>> {
    fn i128(&self) -> Result<&[Option<i128>], &'static str> {
        Ok(&self[..])
    }
}

impl ColumnTrait for Vec<Option<f32>> {
    fn f32(&self) -> Result<&[Option<f32>], &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for Vec<Option<f64>> {
    fn f64(&self) -> Result<&[Option<f64>], &'static str> {
        Ok(&self[..])
    }
}

impl SeriesTrait for Vec<Option<bool>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes) {
        bytes.into_iter().for_each(|words| {
            let el = bytes_to_bool(words);
            self.push(el);
        });
    }

    #[allow(unstable_name_collisions)]
    fn to_js_string(&self, offset: usize, size: usize) -> JsString {
        let s: String = self
            .iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |b| b.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect();
        JsString::from(s)
    }
}

impl ColumnTrait for Vec<Option<bool>> {
    fn bool(&self) -> Result<&[Option<bool>], &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for Vec<Option<String>> {
    fn str(&self) -> Result<&[Option<String>], &'static str> {
        Ok(&self[..])
    }
}

pub struct Column(Box<dyn ColumnTrait>);
pub enum SeriesEnum {
    I32(Box<Vec<Option<i32>>>),
    I64(Box<Vec<Option<i64>>>),
    I128(Box<Vec<Option<i128>>>),
    F32(Box<Vec<Option<f32>>>),
    F64(Box<Vec<Option<f64>>>),
    Bool(Box<Vec<Option<bool>>>),
    Any(Box<Vec<Option<String>>>),
}

impl Column {
    pub fn new(buffer: SeriesEnum) -> Self {
        match buffer {
            SeriesEnum::I32(value) => Self(value),
            SeriesEnum::I64(value) => Self(value),
            SeriesEnum::I128(value) => Self(value),
            SeriesEnum::F32(value) => Self(value),
            SeriesEnum::F64(value) => Self(value),
            SeriesEnum::Bool(value) => Self(value),
            SeriesEnum::Any(value) => Self(value),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn append(&mut self, bytes: ParsedBytes) {
        self.0.parse_and_append_bytes(bytes);
    }

    pub fn to_js_string(&self, offset: usize, size: usize) -> JsString {
        self.0.to_js_string(offset, size)
    }
}
