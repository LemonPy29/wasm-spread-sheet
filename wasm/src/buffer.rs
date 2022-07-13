use core::fmt;

use downcast_rs::{impl_downcast, Downcast};
use itertools::Itertools;
use lexical::{parse, FromLexical};
use num::Num;

use crate::{
    type_parser::{bytes_to_bool, Codes},
    ParsedBytes,
};

const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";
#[derive(Debug)]
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

type FilterFnI32 = &'static dyn Fn(&Option<i32>) -> bool;
type FilterFnI64 = &'static dyn Fn(&Option<i64>) -> bool;
type FilterFnI128 = &'static dyn Fn(&Option<i128>) -> bool;
type FilterFnF32 = &'static dyn Fn(&Option<f32>) -> bool;
type FilterFnF64 = &'static dyn Fn(&Option<f64>) -> bool;
type FilterFnStr = &'static dyn Fn(&Option<String>) -> bool;

pub trait FilterFn: Downcast {}
impl FilterFn for FilterFnI32 {}
impl FilterFn for FilterFnI64 {}
impl FilterFn for FilterFnI128 {}
impl FilterFn for FilterFnF32 {}
impl FilterFn for FilterFnF64 {}
impl FilterFn for FilterFnStr {}
impl_downcast!(FilterFn);

trait SeriesTrait: Downcast {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn parse_and_append_bytes(&mut self, bytes: ParsedBytes);
    fn concat_as_string(&self, offset: usize, size: usize) -> String;
    fn sum_as_series(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        Err("Cannot sum this type")
    }
    fn filter_series_as_string(&self, mask: &mut dyn Iterator<Item = bool>) -> String;
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
    fn filter_column_to_mask<'a>(
        &'a self,
        _pred: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + '_>, WrongType> {
        todo!()
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

    #[allow(unstable_name_collisions)]
    fn filter_series_as_string(&self, mask: &mut dyn Iterator<Item = bool>) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| {
                mask_el.then(|| opt.map_or("".into(), |el| el.to_string()))
            })
            .intersperse(DELIMITER_TOKEN.into())
            .collect::<String>()
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

    #[allow(unstable_name_collisions)]
    fn filter_series_as_string(&self, mask: &mut dyn Iterator<Item = bool>) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| {
                mask_el.then(|| opt.map_or("".into(), |el| el.to_string()))
            })
            .intersperse(DELIMITER_TOKEN.into())
            .collect::<String>()
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

    #[allow(unstable_name_collisions)]
    fn filter_series_as_string(&self, mask: &mut dyn Iterator<Item = bool>) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| mask_el.then(|| opt.as_deref().unwrap_or_default()))
            .intersperse(DELIMITER_TOKEN)
            .collect()
    }
}

macro_rules! filter_column {
    ($column:expr, $filterfn:expr) => {
        Ok(Box::new($column.iter().map(move |el| $filterfn(el))))
    };
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnI32>().unwrap();
        filter_column!(self, filter)
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnI64>().unwrap();
        filter_column!(self, filter)
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnI128>().unwrap();
        filter_column!(self, filter)
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnF32>().unwrap();
        filter_column!(self, filter)
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnF64>().unwrap();
        filter_column!(self, filter)
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

    fn filter_column_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + 'a>, WrongType> {
        let filter = filterfn.downcast_ref::<FilterFnStr>().unwrap();
        filter_column!(self, filter)
    }
}

pub struct Column {
    series: Box<dyn ColumnTrait>,
    name: String,
    dtype: Codes,
}

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
    pub fn new(buffer: SeriesEnum, name: String, dtype: Codes) -> Self {
        match buffer {
            SeriesEnum::I32(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::I64(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::I128(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::F32(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::F64(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::Bool(series) => Self {
                series,
                name,
                dtype,
            },
            SeriesEnum::Any(series) => Self {
                series,
                name,
                dtype,
            },
        }
    }

    pub fn len(&self) -> usize {
        self.series.len()
    }

    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    pub fn append(&mut self, bytes: ParsedBytes) {
        self.series.parse_and_append_bytes(bytes)
    }

    pub fn concat_as_string(&self, offset: usize, size: usize) -> String {
        self.series.concat_as_string(offset, size)
    }

    pub fn sum(&self) -> Result<Self, &str> {
        let series = self.series.sum_as_column()?;
        let name = format!("Sum_of_{}", &self.name);
        Ok(Self {
            series,
            name,
            dtype: self.dtype,
        })
    }

    pub fn first(&self) -> String {
        self.series.concat_as_string(0, 1)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn dtype(&self) -> Codes {
        self.dtype
    }

    pub fn filter_to_mask<'a>(
        &'a self,
        filterfn: &'a dyn FilterFn,
    ) -> Result<Box<dyn Iterator<Item = bool> + '_>, WrongType> {
        self.series.filter_column_to_mask(filterfn)
    }

    pub fn filter_as_string<'a>(&'a self, mask: &'a mut dyn Iterator<Item = bool>) -> String {
        self.series.filter_series_as_string(mask)
    }
}

#[cfg(test)]
mod test {
    use crate::type_parser::Codes;

    use super::{Column, SeriesEnum};

    #[test]
    fn first() {
        let v = vec![Some(1)];
        let series = SeriesEnum::I32(Box::new(v));
        let column = Column::new(series, "_".into(), Codes::Int32);
        let first = column.first();

        assert_eq!(first, "1".to_string());
    }
}
