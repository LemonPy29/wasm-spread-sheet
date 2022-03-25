pub mod buffer;
pub mod csv_parser;
pub mod type_parser;

use buffer::Column;
use console_error_panic_hook::hook;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, panic};
use type_parser::*;
use wasm_bindgen::prelude::*;

pub const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";

pub struct EntryData {
    lines: Vec<String>,
    remainder: Option<Vec<u8>>,
    header: Option<String>,
    n_cols: usize,
    n_chunks: usize,
}

impl EntryData {
    const fn new() -> Self {
        Self {
            lines: Vec::new(),
            remainder: None,
            header: None,
            n_cols: 0,
            n_chunks: 0,
        }
    }

    pub fn view(&self, index: usize) -> &str {
        self.lines[index].as_str()
    }
}

pub static mut ENTRY_DATA: EntryData = EntryData::new();

#[wasm_bindgen]
pub fn add_chunk(chunk: Vec<js_sys::JsString>) {
    unsafe {
        if ENTRY_DATA.lines.is_empty() {
            ENTRY_DATA.lines = chunk.iter().map(|_| String::default()).collect();
            ENTRY_DATA.n_cols = chunk.len();
        }
        ENTRY_DATA
            .lines
            .iter_mut()
            .zip(chunk.iter())
            .for_each(|(line, new)| {
                if ENTRY_DATA.n_chunks > 0 {
                    line.push_str(DELIMITER_TOKEN);
                }
                let s: String = new.into();
                line.push_str(&s)
            });
        ENTRY_DATA.n_chunks += 1;
    }
}

#[wasm_bindgen]
pub fn parse_and_join(chunk: &[u8], skip_header: bool) -> Vec<js_sys::JsString> {
    panic::set_hook(Box::new(hook));

    let mut lines = csv_parser::LineSplitter::from_bytes(chunk);

    unsafe {
        if skip_header && ENTRY_DATA.n_chunks == 0 {
            let _ = lines.next();
        }
    }

    let mut first_line = lines.next();
    let first_chunk = unsafe {
        if let Some(ref mut v) = ENTRY_DATA.remainder {
            let words =
                csv_parser::FieldIter::from_bytes(first_line.expect("Empty buffer")).count();
            if words < ENTRY_DATA.n_cols {
                v.extend_from_slice(first_line.take().expect("Empty buffer"));
            }
            &v[..]
        } else {
            first_line.take().expect("Empty buffer")
        }
    };
    let first_chunk: Vec<&str> = csv_parser::FieldIter::from_bytes(first_chunk).collect();

    let mut ret: Vec<Vec<&str>> = unsafe {
        ENTRY_DATA.n_cols = ENTRY_DATA.n_cols.max(first_chunk.len());
        (0..ENTRY_DATA.n_cols).map(|_| Vec::default()).collect()
    };

    ret.iter_mut()
        .zip(first_chunk.into_iter())
        .for_each(|(v, word)| v.push(word));

    if let Some(v) = first_line {
        let words = csv_parser::FieldIter::from_bytes(v);
        words.enumerate().for_each(|(j, word)| {
            ret[j].push(DELIMITER_TOKEN);
            ret[j].push(word);
        })
    }

    for line in lines {
        let words = csv_parser::FieldIter::from_bytes(line);
        words.enumerate().for_each(|(j, word)| {
            ret[j].push(DELIMITER_TOKEN);
            ret[j].push(word);
        })
    }

    let first_len = ret[0].len();
    let mut last_row: Vec<&str> = Vec::default();
    ret.iter_mut()
        .filter(|v| v.len() == first_len)
        .enumerate()
        .for_each(|(i, v)| {
            if i > 0 {
                last_row.push(",");
            }
            // Pop the last element and append it to the remainder
            last_row.push(v.pop().unwrap());
            // Pop the last token and drop it
            let _ = v.pop();
        });
    let remainder = last_row.concat();

    let ret = ret
        .into_iter()
        .map(|buffer| buffer.concat().into())
        .collect::<Vec<js_sys::JsString>>();

    unsafe {
        ENTRY_DATA.remainder = Some(remainder.as_bytes().to_owned());
        ENTRY_DATA.n_chunks += 1;
    }

    ret
}

#[wasm_bindgen]
pub fn chunks_done() -> usize {
    unsafe { ENTRY_DATA.n_chunks }
}

#[allow(unstable_name_collisions)]
#[wasm_bindgen]
pub fn set_header(chunk: &[u8]) {
    let mut lines = csv_parser::LineSplitter::from_bytes(chunk);

    unsafe {
        ENTRY_DATA.header = lines.next().map(|line| {
            csv_parser::FieldIter::from_bytes(line)
                .intersperse(DELIMITER_TOKEN)
                .collect::<String>()
        });
    }
}

#[wasm_bindgen]
pub fn get_header() -> Option<js_sys::JsString> {
    panic::set_hook(Box::new(hook));
    unsafe {
        let header_ref = ENTRY_DATA.header.as_deref();
        header_ref.map(|s| s.into())
    }
}

#[wasm_bindgen]
pub fn process_remainder() {
    panic::set_hook(Box::new(hook));
    unsafe {
        if let Some(ref rem) = ENTRY_DATA.remainder {
            let words = csv_parser::FieldIter::from_bytes(&rem[..]);
            ENTRY_DATA
                .lines
                .iter_mut()
                .zip(words)
                .for_each(|(line, word)| line.push_str(word))
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub data: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn get_chunk(offset: usize, len: usize) -> JsValue {
    panic::set_hook(Box::new(hook));
    let data = unsafe {
        ENTRY_DATA
            .lines
            .iter()
            .map(|line| {
                line.split(DELIMITER_TOKEN)
                    .skip(offset)
                    .take(len)
                    .map(|s| s.to_string())
                    .collect()
            })
            .collect()
    };

    let chunk = Chunk { data };
    JsValue::from_serde(&chunk).unwrap()
}

#[wasm_bindgen]
pub fn data_len() -> usize {
    panic::set_hook(Box::new(hook));
    unsafe { ENTRY_DATA.lines.len() }
}

#[derive(Serialize, Deserialize)]
pub struct Schema {
    data: HashMap<String, Codes>,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Frame {
    schema: HashMap<String, Codes>,
    columns: Vec<Column>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Frame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut parsed = ParsedWords::default();
        unsafe {
            parsed.write_words(&ENTRY_DATA);
        }
        let mut codes = Vec::new();

        let columns: Vec<Column> = parsed
            .iter_with_code()
            .map(|(code, buffer)| match code {
                code @ Codes::Boolean => {
                    codes.push(code);
                    Column::from_bool(parse_bool(buffer))
                }
                code @ Codes::Int32 => {
                    codes.push(code);
                    Column::new(parse_type::<i32>(buffer))
                }
                code @ Codes::Int64 => {
                    codes.push(code);
                    Column::new(parse_type::<i64>(buffer))
                }
                code @ Codes::Int128 => {
                    codes.push(code);
                    Column::new(parse_type::<i128>(buffer))
                }
                code @ Codes::Float32 => {
                    codes.push(code);
                    Column::new(parse_type::<f32>(buffer))
                }
                code @ Codes::Float64 => {
                    codes.push(code);
                    Column::new(parse_type::<f64>(buffer))
                }
                code @ Codes::Any => {
                    codes.push(code);
                    Column::from_any(parse_utf8(buffer))
                }
                _ => unreachable!(),
            })
            .collect();

        let mut schema = HashMap::new();
        unsafe {
            ENTRY_DATA
                .header
                .as_ref()
                .unwrap()
                .split(DELIMITER_TOKEN)
                .zip(codes.iter())
                .for_each(|(name, code)| {
                    schema.insert(name.to_string(), *code);
                })
        }

        Self { schema, columns }
    }

    #[wasm_bindgen(method, getter = width)]
    pub fn width(&self) -> usize {
        self.columns.len()
    }

    #[wasm_bindgen(method, getter = height)]
    pub fn height(&self) -> usize {
        self.columns.get(0).map_or(0, |v| v.len())
    }

    #[wasm_bindgen(method, getter = schema)]
    pub fn schema(&self) -> JsValue {
        let ret = Schema { data: self.schema.clone() };
        JsValue::from_serde(&ret).unwrap()
    }
}
