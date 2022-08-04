use bitvec::{prelude::BitVec, slice::BitSlice};

use crate::{series::SeriesTrait, type_parser::Codes, Words};
use core::fmt;
use std::collections::HashSet;

#[derive(Debug)]
pub struct WrongType;

impl fmt::Display for WrongType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected type")
    }
}

type View<'a, T> = &'a [Option<T>];
type ViewResult<'a, T> = Result<View<'a, T>, WrongType>;
type FilterResult<'a> = Result<BitVec, WrongType>;

pub trait ColumnTrait: SeriesTrait {
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
    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        Err("Unable to sum type")
    }
    fn equal_to(&self, _col: Box<dyn ColumnTrait>) -> FilterResult {
        Err(WrongType)
    }
}

impl ColumnTrait for Vec<Option<i32>> {
    fn i32(&self) -> ViewResult<i32> {
        Ok(&self[..])
    }

    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .inner_sum()?
            .downcast::<Vec<Option<i32>>>()
            .map_err(|_| "Couldn't downcast to Vec<Option<i32>>")
            .unwrap();
        Ok(sum)
    }

    fn equal_to(&self, col: Box<dyn ColumnTrait>) -> FilterResult {
        let elements = col.i32()?;
        let mut set = HashSet::with_capacity(elements.len());
        for el in elements {
            set.insert(*el);
        }
        let ret: BitVec = self.iter().map(move |el| set.contains(el)).collect();
        Ok(ret)
    }
}

impl ColumnTrait for Vec<Option<i64>> {
    fn i64(&self) -> ViewResult<i64> {
        Ok(&self[..])
    }

    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .inner_sum()?
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

    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .inner_sum()?
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

    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .inner_sum()?
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

    fn outer_sum(&self) -> Result<Box<dyn ColumnTrait>, &str> {
        let sum = self
            .inner_sum()?
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

    fn equal_to(&self, col: Box<dyn ColumnTrait>) -> FilterResult {
        let elements = col.str()?.to_vec();
        let mut set = HashSet::with_capacity(elements.len());
        for el in elements {
            set.insert(el);
        }
        let ret: BitVec = self.iter().map(move |el| set.contains(el)).collect();
        Ok(ret)
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

    pub fn series(&self) -> &dyn ColumnTrait {
        self.series.as_ref()
    }

    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    pub fn extend_from_words(&mut self, bytes: Words) {
        self.series.extend_from_words(bytes)
    }

    pub fn join(&self, offset: usize, size: usize) -> String {
        self.series.join(offset, size)
    }

    pub fn sum(&self) -> Result<Self, &str> {
        let series = self.series.outer_sum()?;
        let name = format!("Sum_of_{}", &self.name);
        Ok(Self {
            series,
            name,
            dtype: self.dtype,
        })
    }

    pub fn first(&self) -> String {
        self.series.join(0, 1)
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn dtype(&self) -> Codes {
        self.dtype
    }

    pub fn equal_to(&self, other: Box<dyn ColumnTrait>) -> FilterResult {
        self.series.equal_to(other)
    }

    pub fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
        self.series.filter_join(mask, offset, size)
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
