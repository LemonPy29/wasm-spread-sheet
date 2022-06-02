#![allow(non_upper_case_globals)]
use crate::Frame;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

static mut FRAME: Frame = Frame::new();

#[wasm_bindgen(js_name = appendChunkToFrame)]
pub fn append_chunk(bytes: &[u8], skip_header: bool, remaining_bytes: &[u8]) -> Vec<u8> {
    unsafe { FRAME.append(bytes, skip_header, remaining_bytes) }
}

#[wasm_bindgen(js_name = getChunk)]
pub fn get_chunk(offset: usize, size: usize) -> Vec<JsString> {
    unsafe {
        FRAME
            .get_chunk(offset, size)
            .iter()
            .map(|s| JsString::from(s.as_str()))
            .collect()
    }
}

#[wasm_bindgen(js_name = appendRemainderToFrame)]
pub fn append_remainder(remainder: &[u8]) {
    unsafe { FRAME.append_remainder(remainder) }
}

#[wasm_bindgen(js_name=getHeader)]
pub fn header() -> Vec<JsString> {
    unsafe { FRAME.header() }
}

#[wasm_bindgen]
pub fn width() -> usize {
    unsafe { FRAME.width() }
}

#[wasm_bindgen]
pub fn height() -> usize {
    unsafe { FRAME.height() }
}

#[wasm_bindgen(js_name = numberOfChunks)]
pub fn n_chunks() -> usize {
    unsafe { FRAME.n_chunks() }
}

#[wasm_bindgen(js_name = sumFrameColumn)]
pub fn sum(index: usize) -> JsString {
    unsafe {
        let s = FRAME
            .columns
            .get(index)
            .expect("Bad index")
            .sum()
            .expect("Unable to sum colunm")
            .first();
        JsString::from(s)
    }
}
