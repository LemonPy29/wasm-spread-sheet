pub mod errors;
pub mod macros;

use bitvec::slice::BitSlice;
use lexical::parse;
use num::Num;

use crate::{
    distinct, equal_to_series, filter_join, join_series, sum_series, type_parser::bytes_to_bool,
    Words,
};

use self::errors::{FilterResult, NonHashable, ViewResult, WrongType};

pub const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";

pub trait Numeric: Copy + Default + Num {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub trait SeriesTrait {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn extend_from_words(&mut self, words: Words);
    fn join(&self, offset: usize, size: usize) -> String;
    fn sum(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        Err("Cannot sum this type")
    }
    fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String;
    fn equal_to(&self, _other: &dyn SeriesTrait) -> FilterResult {
        Err(WrongType)
    }
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
    fn distinct(&self) -> Result<String, NonHashable> {
        Err(NonHashable)
    }
}

impl SeriesTrait for Vec<Option<bool>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn bool(&self) -> ViewResult<bool> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, bytes: Words) {
        bytes.into_iter().for_each(|words| {
            let el = bytes_to_bool(words);
            self.push(el);
        });
    }

    fn join(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |b| b.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect()
    }

    fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| {
                mask_el.then(|| opt.map_or("".into(), |el| el.to_string()))
            })
            .skip(offset)
            .take(size)
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

    fn str(&self) -> ViewResult<String> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, bytes: Words) {
        bytes.into_iter().for_each(|word| {
            let el = String::from_utf8(word.into()).ok();
            self.push(el);
        })
    }

    fn join(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.as_deref().unwrap_or_default())
            .intersperse(DELIMITER_TOKEN)
            .collect()
    }

    fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| mask_el.then(|| opt.as_deref().unwrap_or_default()))
            .skip(offset)
            .take(size)
            .intersperse(DELIMITER_TOKEN)
            .collect()
    }

    equal_to_series!(str);
}

impl SeriesTrait for Vec<Option<i32>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn i32(&self) -> ViewResult<i32> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    join_series!();
    filter_join!();
    sum_series!(i32);
    equal_to_series!(i32);
    distinct!(i32);
}

impl SeriesTrait for Vec<Option<i64>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn i64(&self) -> ViewResult<i64> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    join_series!();
    sum_series!(i64);
    equal_to_series!(i64);
    filter_join!();
}

impl SeriesTrait for Vec<Option<i128>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn i128(&self) -> ViewResult<i128> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    join_series!();
    sum_series!(i128);
    equal_to_series!(i128);
    filter_join!();
}

impl SeriesTrait for Vec<Option<f32>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn f32(&self) -> ViewResult<f32> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    join_series!();
    sum_series!(f32);
    filter_join!();
}

impl SeriesTrait for Vec<Option<f64>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn f64(&self) -> ViewResult<f64> {
        Ok(&self[..])
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    join_series!();
    sum_series!(f64);
    filter_join!();
}
