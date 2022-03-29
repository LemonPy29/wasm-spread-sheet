use num::Num;

pub const BUFFER_SIZE: usize = 1000;

pub enum Writable<'a, T: Copy> {
    Arr(&'a [T]),
    Single(T),
}

pub struct BaseBuffer<T: Default> {
    buffer: [T; BUFFER_SIZE],
    offset: usize,
}

impl<T: Default + Copy> Default for BaseBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + Copy> BaseBuffer<T> {
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); BUFFER_SIZE],
            offset: 0,
        }
    }

    pub fn view(&self, start: usize, end: usize) -> &[T] {
        &self.buffer[start..end]
    }

    pub fn write(&mut self, data: Writable<T>) {
        match data {
            Writable::Arr(slice) => {
                self.buffer[self.offset..slice.len()].copy_from_slice(slice);
                self.offset += slice.len();
            }
            Writable::Single(el) => {
                self.buffer[self.offset] = el;
                self.offset += 1;
            }
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct BufferIter<'a, T: Default> {
    iterable: &'a BaseBuffer<T>,
    start: usize,
    end: usize,
}

impl<'a, T: Default + 'a> Iterator for BufferIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.start += 1;
        (self.start <= self.end).then(|| &self.iterable.buffer[self.start])
    }
}

impl<'a, T: Default + 'a> IntoIterator for &'a BaseBuffer<T> {
    type Item = &'a T;
    type IntoIter = BufferIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        BufferIter {
            iterable: self,
            start: 0,
            end: self.offset,
        }
    }
}

pub trait Numeric: Copy + Default + std::str::FromStr + Num {}

impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

trait SeriesTrait: Send + Sync {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

trait ColumnTrait: SeriesTrait {
    fn i32(&self) -> Result<&BaseBuffer<Option<i32>>, &str> {
        Err("Unexpected type")
    }
    fn i64(&self) -> Result<&BaseBuffer<Option<i64>>, &str> {
        Err("Unexpected type")
    }
    fn i128(&self) -> Result<&BaseBuffer<Option<i128>>, &str> {
        Err("Unexpected type")
    }
    fn f32(&self) -> Result<&BaseBuffer<Option<f32>>, &str> {
        Err("Unexpected type")
    }
    fn f64(&self) -> Result<&BaseBuffer<Option<f64>>, &str> {
        Err("Unexpected type")
    }
    fn bool(&self) -> Result<&BaseBuffer<Option<bool>>, &str> {
        Err("Unexpected type")
    }
    fn str(&self) -> Result<&BaseBuffer<Option<&str>>, &str> {
        Err("Unexpected type")
    }
}

impl<T: Numeric + Send + Sync> SeriesTrait for BaseBuffer<Option<T>> {
    fn len(&self) -> usize {
        self.get_offset()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl ColumnTrait for BaseBuffer<Option<i32>> {
    fn i32(&self) -> Result<&BaseBuffer<Option<i32>>, &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for BaseBuffer<Option<i64>> {
    fn i64(&self) -> Result<&BaseBuffer<Option<i64>>, &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for BaseBuffer<Option<i128>> {
    fn i128(&self) -> Result<&BaseBuffer<Option<i128>>, &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for BaseBuffer<Option<f32>> {
    fn f32(&self) -> Result<&BaseBuffer<Option<f32>>, &'static str> {
        Ok(self)
    }
}

impl ColumnTrait for BaseBuffer<Option<f64>> {
    fn f64(&self) -> Result<&BaseBuffer<Option<f64>>, &'static str> {
        Ok(self)
    }
}

impl SeriesTrait for BaseBuffer<Option<bool>> {
    fn len(&self) -> usize {
        self.get_offset()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl ColumnTrait for BaseBuffer<Option<bool>> {
    fn bool(&self) -> Result<&BaseBuffer<Option<bool>>, &'static str> {
        Ok(self)
    }
}
impl SeriesTrait for BaseBuffer<Option<&str>> {
    fn len(&self) -> usize {
        self.get_offset()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl ColumnTrait for BaseBuffer<Option<&str>> {
    fn str(&self) -> Result<&BaseBuffer<Option<&str>>, &'static str> {
        Ok(self)
    }
}

pub struct Column(Box<dyn ColumnTrait>);
pub enum SeriesEnum {
    I32(Box<BaseBuffer<Option<i32>>>),
    I64(Box<BaseBuffer<Option<i64>>>),
    I128(Box<BaseBuffer<Option<i128>>>),
    F32(Box<BaseBuffer<Option<f32>>>),
    F64(Box<BaseBuffer<Option<f64>>>),
    Bool(Box<BaseBuffer<Option<bool>>>),
    Any(Box<BaseBuffer<Option<&'static str>>>),
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
}

#[cfg(test)]
mod test {
    use super::{BaseBuffer, Writable};

    #[test]
    fn parse() {
        let mut buffer = BaseBuffer::new();
        buffer.write(Writable::Arr(&["1", "2", "3"]));
        assert_eq!(buffer.get_offset(), 3);
    }

    #[test]
    fn write_arr() {
        let mut buffer = BaseBuffer::new();
        buffer.write(Writable::Arr(&[Some(1); 50]));
        assert_eq!(buffer.get_offset(), 50);
    }

    #[test]
    fn write_single() {
        let mut buffer = BaseBuffer::new();
        for _ in 0..100 {
            buffer.write(Writable::Single(1));
        }
        assert_eq!(buffer.get_offset(), 100);
    }
}
