#![allow(non_upper_case_globals)]
use crate::{filter::Filter, Frame};
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
}

#[wasm_bindgen]
impl Filter {
    #[wasm_bindgen(method, js_name = slice)]
    pub fn slice(
        &self,
        frame: &Frame,
        offset: usize,
        size: usize,
    ) -> Vec<JsString> {
        let mask = self.get();
        frame
            .columns
            .iter()
            .map(|col| col.filter_join(mask, offset, size))
            .map(|s| JsString::from(s.as_str()))
            .collect()
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
