#![allow(non_upper_case_globals)]
use crate::Frame;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = newFrame)]
pub fn new_frame() -> Frame {
    Frame::new()
}

#[wasm_bindgen]
pub fn width(frame: &Frame) -> usize {
    frame.width()
}

#[wasm_bindgen]
pub fn height(frame: &Frame) -> usize {
    frame.height()
}

#[wasm_bindgen(js_name = numberOfChunks)]
pub fn n_chunks(frame: &Frame) -> usize {
    frame.n_chunks()
}

#[wasm_bindgen(js_name = readPushStreamChunk)]
pub fn read_and_push_stream_chunk(
    frame: &mut Frame,
    bytes: &[u8],
    skip_header: bool,
    remaining_bytes: &[u8],
) -> Vec<u8> {
    frame.append(bytes, skip_header, remaining_bytes)
}

#[wasm_bindgen(js_name = readPushRemainingStream)]
pub fn read_and_push_remaining_stream(frame: &mut Frame, remainder: &[u8]) {
    frame.append_remainder(remainder)
}

#[wasm_bindgen(js_name = sliceAsJsStrings)]
pub fn slice_as_js_strings(frame: &mut Frame, offset: usize, size: usize) -> Vec<JsString> {
    frame
        .get_chunk(offset, size)
        .iter()
        .map(|s| JsString::from(s.as_str()))
        .collect()
}

#[wasm_bindgen(js_name=getHeader)]
pub fn get_header(frame: &Frame) -> Vec<JsString> {
    frame.header().map(JsString::from).collect()
}

#[wasm_bindgen(js_name=getDtypes)]
pub fn get_dtypes(frame: &Frame) -> Vec<JsString> {
    frame.dtypes().map(JsString::from).collect()
}

#[wasm_bindgen(js_name = sumFrameColumn)]
pub fn sum(frame: &Frame, index: usize) -> JsString {
    let s = frame
        .columns
        .get(index)
        .expect("Bad index")
        .sum()
        .expect("Unable to sum colunm")
        .first();
    JsString::from(s)
}
//
// #[wasm_bindgen(js_name = equalToFilter)]
// pub fn equal_to(frame: &Frame, name: &str, bytes: &[u8]) -> Mask {
//     Mask::to_equal(frame, name, bytes)
// }

