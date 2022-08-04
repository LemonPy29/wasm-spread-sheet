use bitvec::slice::BitSlice;
use downcast_rs::{impl_downcast, Downcast};
use lexical::{parse, FromLexical};
use num::Num;

use crate::{type_parser::bytes_to_bool, Words};

const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";
pub trait Numeric: Copy + Default + Num {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f32 {}
impl Numeric for f64 {}

pub trait SeriesTrait: Downcast {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn extend_from_words(&mut self, words: Words);
    fn join(&self, offset: usize, size: usize) -> String;
    fn inner_sum(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        Err("Cannot sum this type")
    }
    fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String;
}

impl_downcast!(SeriesTrait);

impl<T: Numeric + FromLexical> SeriesTrait for Vec<Option<T>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn extend_from_words(&mut self, words: Words) {
        words.into_iter().for_each(|word| {
            let el = parse(word).ok();
            self.push(el);
        })
    }

    fn join(&self, offset: usize, size: usize) -> String {
        self.iter()
            .skip(offset)
            .take(size)
            .map(|opt| opt.map_or("".into(), |number| number.to_string()))
            .intersperse(DELIMITER_TOKEN.into())
            .collect()
    }

    fn inner_sum(&self) -> Result<Box<dyn SeriesTrait>, &str> {
        let sum = self
            .iter()
            .fold(T::default(), |acc, x| acc + x.unwrap_or_default());
        let series = vec![Some(sum)];
        Ok(Box::new(series))
    }

    fn filter_join(&self, mask: &BitSlice, offset: usize, size: usize) -> String {
        self.iter()
            .zip(mask)
            .filter_map(|(opt, mask_el)| mask_el.then(|| opt.map_or("".into(), |el| el.to_string())))
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
            .filter_map(|(opt, mask_el)| {
                mask_el.then(|| opt.as_deref().unwrap_or_default())
            })
            .skip(offset)
            .take(size)
            .intersperse(DELIMITER_TOKEN)
            .collect()
    }
}

impl SeriesTrait for Vec<Option<bool>> {
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
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
