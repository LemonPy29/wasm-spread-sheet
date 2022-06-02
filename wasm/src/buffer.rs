use core::fmt;

use downcast_rs::{impl_downcast, Downcast};
use itertools::Itertools;
use lexical::{parse, FromLexical};
use num::Num;

use crate::{type_parser::bytes_to_bool, ParsedBytes};

const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";
pub struct WrongType;

impl fmt::Display for WrongType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected type")
    }
}

pub trait Numeric: Copy + Default + Num {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

trait SeriesTrait: Downcast {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes);
    fn concat_as_string(&self, offset: usize, size: usize) -> String;
    fn sum_as_series(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        Err("Cannot sum this type")
    }
}
impl_downcast!(SeriesTrait);

type View<'a, T> = &'a [Option<T>];
type ViewResult<'a, T> = Result<View<'a, T>, WrongType>;

trait ColumnTrait: SeriesTrait {
    fn i32(&self) -> ViewResult<i32> {
        Err(WrongType)
    }
    fn i64(&self) -> ViewResult<i64> {
        Err(WrongType)
    }
    fn i128(&self) -> ViewResult<i128> {
        Err(WrongType)
    }
    fn f32(&self) -> ViewResult<f32> {
        Err(WrongType)
    }
    fn f64(&self) -> ViewResult<f64> {
        Err(WrongType)
    }
    fn bool(&self) -> ViewResult<bool> {
        Err(WrongType)
    }
    fn str(&self) -> ViewResult<String> {
        Err(WrongType)
    }
    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        Err("Unable to sum type")
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
    fn concat_as_string(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |number| number.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect()
    }

    fn sum_as_series(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        let sum = self
            .iter()
            .fold(T::default(), |acc, x| acc + x.unwrap_or_default());
        let series = vec![Some(sum)];
        Ok(Box::new(series))
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
    fn concat_as_string(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |b| b.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect()
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
    fn concat_as_string(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.as_deref().unwrap_or_default())
            .intersperse(DELIMITER_TOKEN)
            .collect()
    }
}

impl ColumnTrait for Vec<Option<i32>> {
    fn i32(&self) -> ViewResult<i32> {
        Ok(&self[..])
    }
    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .sum_as_series()?
            .downcast::<Vec<Option<i32>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }
}

impl ColumnTrait for Vec<Option<i64>> {
    fn i64(&self) -> ViewResult<i64> {
        Ok(&self[..])
    }

    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .sum_as_series()?
            .downcast::<Vec<Option<i64>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }
}

impl ColumnTrait for Vec<Option<i128>> {
    fn i128(&self) -> ViewResult<i128> {
        Ok(&self[..])
    }

    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .sum_as_series()?
            .downcast::<Vec<Option<i128>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }
}

impl ColumnTrait for Vec<Option<f32>> {
    fn f32(&self) -> ViewResult<f32> {
        Ok(self)
    }

    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .sum_as_series()?
            .downcast::<Vec<Option<f32>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }
}

impl ColumnTrait for Vec<Option<f64>> {
    fn f64(&self) -> ViewResult<f64> {
        Ok(&self[..])
    }

    fn sum_as_column(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .sum_as_series()?
            .downcast::<Vec<Option<f64>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }
}

impl ColumnTrait for Vec<Option<bool>> {
    fn bool(&self) -> ViewResult<bool> {
        Ok(&self[..])
    }
}

impl ColumnTrait for Vec<Option<String>> {
    fn str(&self) -> ViewResult<String> {
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
        let Column(inn) = self;
        inn.len()
    }

    pub fn is_empty(&self) -> bool {
        let Column(inn) = self;
        inn.is_empty()
    }

    pub fn append(&mut self, bytes: ParsedBytes) {
        let Column(inn) = self;
        inn.parse_and_append_bytes(bytes)
    }

    pub fn concat_as_string(&self, offset: usize, size: usize) -> String {
        let Column(inn) = self;
        inn.concat_as_string(offset, size)
    }

    pub fn sum(&self) -> Result<Self, &str> {
        let Column(inn) = self;
        let sum = inn.sum_as_column()?;
        Ok(Self(sum))
    }

    pub fn first(&self) -> String {
        let Column(inn) = self;
        inn.concat_as_string(0, 1)
    }
}

#[cfg(test)]
mod test {
    use super::{Column, SeriesEnum};

    #[test]
    fn first() {
        let v = vec![Some(1)];
        let series = SeriesEnum::I32(Box::new(v));
        let column = Column::new(series);
        let first = column.first();

        assert_eq!(first, "1".to_string());
    }
}
