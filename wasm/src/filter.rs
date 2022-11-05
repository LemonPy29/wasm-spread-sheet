use bitvec::{prelude::BitVec, slice::BitSlice};
use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    column::ColumnTrait,
    csv_parser::FieldIter,
    type_parser::{parse_type, parse_utf8, Codes},
    Frame, Words,
};

pub fn single_buffer_into_col_trait(bytes: &[u8], code: Codes) -> Box<dyn ColumnTrait> {
    let mut commands = Words::default();
    let words = FieldIter::from_bytes(bytes);
    for word in words {
        commands.extend(word);
    }

    match code {
        Codes::Int32 => Box::new(parse_type::<i32>(commands)),
        Codes::Any => Box::new(parse_utf8(commands)),
        _ => panic!("Unimplemented"),
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Filter {
    filter: BitVec,
}

impl Filter {
    pub fn add_equalto_filter(&mut self, frame: &Frame, bytes: &[u8], column: &str) {
        let col = frame.find_by_name(column);
        let other = single_buffer_into_col_trait(bytes, col.dtype());
        let mask = col.equal_to(other).unwrap();

        self.filter = mask;
    }

    fn get(&self) -> &BitSlice {
        self.filter.as_bitslice()
    }

    pub fn slice(&self, frame: &Frame, offset: usize, size: usize) -> Vec<JsString> {
        let mask = self.get();
        frame
            .columns
            .iter()
            .map(|col| col.filter_join(mask, offset, size))
            .map(|s| JsString::from(s.as_str()))
            .collect()
    }

}
