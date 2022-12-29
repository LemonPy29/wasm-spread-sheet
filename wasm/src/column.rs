use bitvec::slice::BitSlice;

use crate::{
    series::{
        errors::{FilterResult, NonHashable},
        SeriesTrait,
    },
    type_parser::Codes,
    Words,
};

pub struct Column {
    series: Box<dyn SeriesTrait>,
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

    pub fn series(&self) -> &dyn SeriesTrait {
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
        let series = self.series.sum()?;
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

    pub fn equal_to(&self, other: &dyn SeriesTrait) -> FilterResult {
        self.series.equal_to(other)
    }

    pub fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
        self.series.filter_join(mask, offset, size)
    }

    pub fn distinct(&self) -> Result<String, NonHashable> {
        self.series.distinct()
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
