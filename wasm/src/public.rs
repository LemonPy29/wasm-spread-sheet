#![allow(non_upper_case_globals)]
use crate::{
    command::exec::{exec, Slice},
    filter::Filter,
    Frame,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Frame {
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.columns.len()
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.columns.get(0).map_or(0, |v| v.len())
    }

    #[wasm_bindgen(getter)]
    pub fn header(&self) -> Vec<JsString> {
        self.columns
            .iter()
            .map(|column| column.name())
            .map(JsString::from)
            .collect()
    }

    #[wasm_bindgen(getter = numberOfChunks)]
    pub fn n_chunks(&self) -> usize {
        self.n_chunks
    }

    #[wasm_bindgen(getter = dtypes)]
    pub fn dtypes(&self) -> Vec<JsString> {
        self.columns
            .iter()
            .map(|column| column.dtype())
            .map(JsString::from)
            .collect()
    }

    #[wasm_bindgen(method)]
    pub fn slice(&self, offset: usize, size: usize) -> Vec<JsString> {
        self.columns
            .iter()
            .map(|column| column.join(offset, size))
            .map(|s| JsString::from(s.as_str()))
            .collect()
    }

    #[wasm_bindgen(method)]
    pub fn distinct(&self, column: &str) -> Result<JsString, JsString> {
        let value = self
            .find_by_name(column)
            .distinct()
            .map_err(|_| JsString::from("Cannot Hash Type"))?;
        let ret = JsString::from(value.as_str());
        Ok(ret)
    }
}

#[wasm_bindgen]
pub struct PollSource {
    _type: &'static str,
    source: Slice,
}

#[wasm_bindgen]
impl PollSource {
    #[wasm_bindgen(method)]
    pub fn slice(&self, frame: &Frame, offset: usize, size: usize) -> Vec<JsString> {
        match &self.source {
            Slice::FilterSlice(filter) => filter.slice(frame, offset, size),
        }
    }

    pub fn source_type(&self) -> JsString {
        JsString::from(self._type)
    }
}

#[wasm_bindgen(js_name = processCommand)]
pub fn process_command(input: &str, frame: &Frame) -> PollSource {
    let slice = exec(input, frame).unwrap();
    match slice {
        Slice::FilterSlice(_) => PollSource {
            _type: "filter",
            source: slice,
        },
    }
}

#[wasm_bindgen(js_name = newFilter)]
pub fn new() -> Filter {
    Filter::default()
}

#[wasm_bindgen(js_name = newFrame)]
pub fn new_frame() -> Frame {
    Frame::new()
}

#[wasm_bindgen(js_name = processStreamChunk)]
pub fn process_stream_chunk(frame: &mut Frame, bytes: &[u8], skip_header: bool) {
    frame.append(bytes, skip_header);
}

#[wasm_bindgen(js_name = processStreamTail)]
pub fn process_stream_tail(frame: &mut Frame) {
    frame.append_remainder();
}

#[wasm_bindgen(js_name = addEqualtoFilter)]
pub fn add_equalto_filter(filter: &mut Filter, frame: &Frame, bytes: &[u8], column: &str) {
    filter.add_equalto_filter(frame, bytes, column)
}
