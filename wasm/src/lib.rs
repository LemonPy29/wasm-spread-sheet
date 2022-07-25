#![feature(generic_associated_types, iter_intersperse)]
pub mod column;
pub mod csv_parser;
pub mod public;
pub mod series;
pub mod type_parser;
pub mod utils;
pub mod filters;

use column::{Column, SeriesEnum};
use console_error_panic_hook::hook;
use csv_parser::LineSplitter;
use std::panic;
use type_parser::*;
use utils::{HeaderFillerGenerator, LendingIterator};
use wasm_bindgen::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ParsedBytes {
    buff: Vec<u8>,
    offsets: Vec<usize>,
}

impl ParsedBytes {
    pub fn last(&self) -> Option<usize> {
        self.offsets.last().copied()
    }

    pub fn extend(&mut self, data: &[u8]) {
        self.buff.extend_from_slice(data);
        if let Some(current) = self.last() {
            self.offsets.push(current + data.len());
        } else {
            self.offsets.push(data.len());
        }
    }

    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }

    pub fn pop_at_last_offset(&mut self) -> Vec<u8> {
        let l = self.offsets.len() - 1;
        let second_to_last = self.offsets[l - 1];
        let _ = self.offsets.pop();
        self.buff.drain(second_to_last..).collect()
    }
}

pub struct ParsedBytesIter<'a> {
    iter: &'a ParsedBytes,
    cursor: usize,
}

impl<'a> Iterator for ParsedBytesIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.iter.offsets.len() {
            return None;
        }

        match self.cursor {
            0 => {
                self.cursor += 1;
                self.iter
                    .offsets
                    .first()
                    .map(|first| &self.iter.buff[..*first])
            }
            _ => {
                let (end, start) = (
                    self.iter.offsets[self.cursor],
                    self.iter.offsets[self.cursor - 1],
                );
                self.cursor += 1;
                Some(&self.iter.buff[start..end])
            }
        }
    }
}

impl<'a> IntoIterator for &'a ParsedBytes {
    type Item = &'a [u8];
    type IntoIter = ParsedBytesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ParsedBytesIter {
            iter: self,
            cursor: 0,
        }
    }
}

pub struct ChunkFromJsBytes {
    buffers: Vec<ParsedBytes>,
    remainder: Option<Vec<u8>>,
    header: Option<ParsedBytes>,
}

impl ChunkFromJsBytes {
    fn from_bytes(bytes: &[u8]) -> ChunkBuilder {
        let mut v = Vec::with_capacity(bytes.len());
        v.extend_from_slice(bytes);
        ChunkBuilder {
            bytes: v,
            missing_bytes: None,
            skip_header: false,
            n_cols: 0,
        }
    }

    fn generate_codes(&self) -> Vec<Codes> {
        panic::set_hook(Box::new(hook));
        let infer_size: usize = (self.buffers[0].len() as f32 * 0.1) as usize;
        let n_words = infer_size.max(1);

        self.buffers
            .iter()
            .map(move |buffer| {
                let code: Codes = buffer
                    .into_iter()
                    .take(n_words)
                    .map(|bytes| {
                        let word = std::str::from_utf8(bytes).expect("Invalid bytes");
                        match first_phase(word) {
                            StageOne::Int(text) => IntegerTypes::from(text).into(),
                            StageOne::Float(text) => FloatTypes::from(text).into(),
                            StageOne::Any(text) if text.is_empty() => Codes::Null,
                            val @ StageOne::Boolean(_) | val @ StageOne::Any(_) => val.into(),
                        }
                    })
                    .max()
                    .unwrap();
                code
            })
            .collect()
    }

    fn iter_with_code(self) -> impl Iterator<Item = (Codes, ParsedBytes)> {
        let codes = self.generate_codes();
        codes.into_iter().zip(self.buffers.into_iter())
    }

    pub fn pull_last_line(mut self) -> Self {
        panic::set_hook(Box::new(hook));
        let first_len = self.buffers[0].len();
        let mut remainder: Vec<u8> = Vec::default();
        self.buffers
            .iter_mut()
            .filter(|v| v.len() == first_len)
            .enumerate()
            .for_each(|(i, v)| {
                if i > 0 {
                    remainder.push(b',');
                }

                let word = v.pop_at_last_offset();
                remainder.extend_from_slice(&word);
            });
        self.remainder = Some(remainder);
        self
    }

    fn single_line(bytes: &[u8], n_cols: usize) -> Self {
        let words = csv_parser::FieldIter::from_bytes(bytes);
        let mut buffers: Vec<ParsedBytes> = (0..n_cols).map(|_| ParsedBytes::default()).collect();

        buffers
            .iter_mut()
            .zip(words)
            .for_each(|(v, word)| v.extend(word));

        Self {
            buffers,
            header: None,
            remainder: None,
        }
    }

    fn fill_header(&mut self) -> ParsedBytes {
        let ret = self.header.take();

        ret.unwrap_or_else(|| {
            let mut filler_generator = HeaderFillerGenerator::<u8>::default();
            let mut fallback = ParsedBytes::default();

            for _ in 0..self.buffers.len() {
                let name = filler_generator.next().expect("Maximum columns exceeded");
                fallback.extend(name);
            }

            fallback
        })
    }
}

struct ChunkBuilder {
    bytes: Vec<u8>,
    missing_bytes: Option<Vec<u8>>,
    skip_header: bool,
    n_cols: usize,
}

impl ChunkBuilder {
    fn with_header(&mut self, val: bool) -> &mut Self {
        self.skip_header = val;
        self
    }

    fn with_missing_bytes(&mut self, bytes: Option<Vec<u8>>) -> &mut Self {
        self.missing_bytes = bytes;
        self
    }

    fn with_column_number(&mut self, n_cols: usize) -> &mut Self {
        self.n_cols = n_cols;
        self
    }

    fn read(&mut self) -> ChunkFromJsBytes {
        panic::set_hook(Box::new(hook));

        let mut lines = LineSplitter::from_bytes(self.bytes.as_slice());

        let header = if self.skip_header {
            let line = lines.next().expect("Empty buffer");
            let words = csv_parser::FieldIter::from_bytes(line);
            let mut parsed = ParsedBytes::default();

            words.for_each(|word| parsed.extend(word));
            Some(parsed)
        } else {
            None
        };

        let mut first_line = lines.next();
        let first_chunk = if let Some(ref mut v) = self.missing_bytes {
            let words =
                csv_parser::FieldIter::from_bytes(first_line.expect("Empty buffer")).count();
            if words < self.n_cols {
                v.extend_from_slice(first_line.take().expect("Empty buffer"));
            }
            &v[..]
        } else {
            first_line.take().expect("Empty buffer")
        };

        let first_chunk: Vec<&[u8]> = csv_parser::FieldIter::from_bytes(first_chunk).collect();

        let width = self.n_cols.max(first_chunk.len());
        let mut buffers: Vec<ParsedBytes> = (0..width).map(|_| ParsedBytes::default()).collect();

        buffers
            .iter_mut()
            .zip(first_chunk.into_iter())
            .for_each(|(v, word)| v.extend(word));

        if let Some(v) = first_line {
            let words = csv_parser::FieldIter::from_bytes(v);
            words.enumerate().for_each(|(j, word)| {
                buffers[j].extend(word);
            })
        }

        for line in lines {
            let words = csv_parser::FieldIter::from_bytes(line);
            words.enumerate().for_each(|(j, word)| {
                buffers[j].extend(word);
            })
        }

        ChunkFromJsBytes {
            buffers,
            remainder: None,
            header,
        }
    }
}

#[wasm_bindgen]
pub struct Frame {
    index: Vec<usize>,
    columns: Vec<Column>,
    n_chunks: usize,
}

#[allow(clippy::new_without_default)]
impl Frame {
    fn new() -> Self {
        Self {
            index: Vec::new(),
            columns: Vec::new(),
            n_chunks: 0,
        }
    }

    fn new_from_entry(&mut self, mut entry: ChunkFromJsBytes) {
        let header = entry.fill_header();

        self.columns = entry
            .iter_with_code()
            .zip(header.into_iter())
            .map(|((code, words), name_bytes)| match code {
                code @ Codes::Boolean => {
                    let parsed = parse_bool(words);
                    let series = SeriesEnum::Bool(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Int32 => {
                    let parsed = parse_type::<i32>(words);
                    let series = SeriesEnum::I32(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Int64 => {
                    let parsed = parse_type::<i64>(words);
                    let series = SeriesEnum::I64(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Int128 => {
                    let parsed = parse_type::<i128>(words);
                    let series = SeriesEnum::I128(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Float32 => {
                    let parsed = parse_type::<f32>(words);
                    let series = SeriesEnum::F32(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Float64 => {
                    let parsed = parse_type::<f64>(words);
                    let series = SeriesEnum::F64(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                code @ Codes::Any => {
                    let parsed = parse_utf8(words);
                    let series = SeriesEnum::Any(Box::new(parsed));
                    let name = String::from_utf8(name_bytes.to_vec()).unwrap();
                    Column::new(series, name, code)
                }
                _ => unreachable!(),
            })
            .collect();

        if let Some(v) = self.columns.get(0) {
            self.index = (0..v.len()).collect();
        }
    }

    fn extend_from_buffers(&mut self, buffers: Vec<ParsedBytes>) {
        self.columns
            .iter_mut()
            .zip(buffers.into_iter())
            .for_each(|(col, buff)| col.append(buff));
    }

    pub fn append_line(&mut self, bytes: &[u8]) {
        let chunk = ChunkFromJsBytes::single_line(bytes, self.columns.len());
        self.extend_from_buffers(chunk.buffers);
    }

    pub fn append(&mut self, bytes: &[u8], skip_header: bool, remaining_bytes: &[u8]) -> Vec<u8> {
        panic::set_hook(Box::new(hook));

        let old_rem = (!remaining_bytes.is_empty()).then(|| remaining_bytes.to_owned());
        let chunk = ChunkFromJsBytes::from_bytes(bytes)
            .with_missing_bytes(old_rem)
            .with_header(skip_header && self.n_chunks == 0)
            .with_column_number(self.columns.len())
            .read()
            .pull_last_line();

        let ret = chunk.remainder.clone().unwrap_or_default();
        if self.columns.is_empty() {
            self.new_from_entry(chunk);
        } else {
            self.extend_from_buffers(chunk.buffers);
        };

        self.n_chunks += 1;
        ret
    }

    pub fn append_remainder(&mut self, line: &[u8]) {
        self.append_line(line);
    }

    pub fn get_chunk(&self, offset: usize, size: usize) -> Vec<String> {
        panic::set_hook(Box::new(hook));
        self.columns
            .iter()
            .map(|column| column.concat_as_string(offset, size))
            .collect()
    }

    pub fn n_chunks(&self) -> usize {
        self.n_chunks
    }

    pub fn width(&self) -> usize {
        self.columns.len()
    }

    pub fn height(&self) -> usize {
        self.columns.get(0).map_or(0, |v| v.len())
    }

    pub fn header(&self) -> impl Iterator<Item = &str> {
        self.columns.iter().map(|column| column.name())
    }

    pub fn dtypes(&self) -> impl Iterator<Item = Codes> + '_ {
        self.columns.iter().map(|column| column.dtype())
    }

    pub fn find_by_name(&self, name: &str) -> &Column {
        self.columns.iter().find(|&col| col.name() == name).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_bytes() {
        let one = "Flareon".as_bytes();
        let two = "Jolteon".as_bytes();
        let three = "Vaporeon".as_bytes();

        let mut parsed = ParsedBytes::default();
        parsed.extend(one);
        assert_eq!(parsed.len(), 1);

        parsed.extend(two);
        parsed.extend(three);

        let mut iter = parsed.into_iter();
        assert_eq!(iter.next(), Some(one));
        assert_eq!(iter.next(), Some(two));
        assert_eq!(iter.next(), Some(three));

        assert_eq!(parsed.pop_at_last_offset(), three);
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn bytes_into_chunk() {
        let bytes = "Flareon,Jolteon,Vaporeon\nEsp".as_bytes();
        let ChunkFromJsBytes {
            buffers,
            remainder,
            header,
        } = ChunkFromJsBytes::from_bytes(bytes).read().pull_last_line();

        assert_eq!(header, None);
        assert_eq!(buffers.len(), 3);
        assert_eq!(remainder, Some("Esp".as_bytes().into()));
    }

    #[test]
    fn frame() {
        let bytes = "FieldOne,FieldTwo,FieldThree\nFlareon,2.5,1\nVaporeon,1.2,2".as_bytes();
        let chunk = ChunkFromJsBytes::from_bytes(bytes).with_header(true).read();
        let mut frame = Frame::new();

        frame.new_from_entry(chunk);
        assert_eq!(frame.width(), 3);
        assert_eq!(frame.height(), 2);

        let last = "Jolteon,1.5,3".as_bytes();
        frame.append_line(last);
        assert_eq!(frame.height(), 3);
    }
}
