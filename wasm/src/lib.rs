pub mod buffer;
pub mod csv_parser;
pub mod type_parser;

use buffer::{Column, SeriesEnum};
use console_error_panic_hook::hook;
use csv_parser::LineSplitter;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use web_sys::console;
use std::{collections::HashMap, panic};
use type_parser::*;
use wasm_bindgen::prelude::*;

pub const DELIMITER_TOKEN: &str = "DELIMITER_TOKEN";

pub struct EntryData {
    lines: Vec<Vec<String>>,
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

    pub fn view(&self, index: usize) -> &[String] {
        self.lines[index].as_slice()
    }
}

pub static mut ENTRY_DATA: EntryData = EntryData::new();

#[wasm_bindgen]
pub fn parse_lines(chunk: &[u8],  skip_header: bool) {
    panic::set_hook(Box::new(hook));

    let mut lines = LineSplitter::from_bytes(chunk);

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

    unsafe {
        ENTRY_DATA.n_cols = ENTRY_DATA.n_cols.max(first_chunk.len());

        if ENTRY_DATA.lines.is_empty() {
            ENTRY_DATA.lines = (0..ENTRY_DATA.n_cols).map(|_| Vec::default()).collect();
        }

        ENTRY_DATA
            .lines
            .iter_mut()
            .zip(first_chunk.into_iter())
            .for_each(|(v, word)| v.push(word.into()));
    }

    unsafe {
        if let Some(v) = first_line {
            let words = csv_parser::FieldIter::from_bytes(v);
            words.enumerate().for_each(|(j, word)| {
                ENTRY_DATA.lines[j].push(word.into());
            })
        }
    }

    unsafe {
        for line in lines {
            let words = csv_parser::FieldIter::from_bytes(line);
            words.enumerate().for_each(|(j, word)| {
                ENTRY_DATA.lines[j].push(word.into());
            })
        }
    }

    let remainder = unsafe {
        let first_len = ENTRY_DATA.lines[0].len();
        let mut last_row: Vec<String> = Vec::default();
        ENTRY_DATA
            .lines
            .iter_mut()
            .filter(|v| v.len() == first_len)
            .enumerate()
            .for_each(|(i, v)| {
                if i > 0 {
                    last_row.push(",".into());
                }
                // Pop the last element and append it to the remainder
                last_row.push(v.pop().unwrap());
                // Pop the last token and drop it
                let _ = v.pop();
            });
        last_row.concat()
    };

    unsafe {
        ENTRY_DATA.remainder = Some(remainder.as_bytes().to_owned());
        ENTRY_DATA.n_chunks += 1;
    }
}

#[wasm_bindgen]
pub fn progress() -> usize {
    let ret = unsafe { ENTRY_DATA.n_chunks };
    console::log_2(&"N cols from rust".into(), &ret.into());
    ret
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
                .for_each(|(line, word)| line.push(word.into()))
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
                line.iter()
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
                    let parsed = parse_bool(buffer);
                    let series = SeriesEnum::Bool(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Int32 => {
                    codes.push(code);
                    let parsed = parse_type::<i32>(buffer);
                    let series = SeriesEnum::I32(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Int64 => {
                    codes.push(code);
                    let parsed = parse_type::<i64>(buffer);
                    let series = SeriesEnum::I64(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Int128 => {
                    codes.push(code);
                    let parsed = parse_type::<i128>(buffer);
                    let series = SeriesEnum::I128(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Float32 => {
                    codes.push(code);
                    let parsed = parse_type::<f32>(buffer);
                    let series = SeriesEnum::F32(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Float64 => {
                    codes.push(code);
                    let parsed = parse_type::<f64>(buffer);
                    let series = SeriesEnum::F64(Box::new(parsed));
                    Column::new(series)
                }
                code @ Codes::Any => {
                    codes.push(code);
                    let parsed = parse_utf8(buffer);
                    let series = SeriesEnum::Any(Box::new(parsed));
                    Column::new(series)
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
        let ret = Schema {
            data: self.schema.clone(),
        };
        JsValue::from_serde(&ret).unwrap()
    }
}
