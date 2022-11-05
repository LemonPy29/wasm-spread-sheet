use crate::{Frame, filter::Filter};
use super::parser::{parse_command, ParsedCommand};

pub enum Slice {
    FilterSlice(Filter)
}

pub fn exec(input: &str, frame: &Frame) -> Result<Slice, &'static str> {
    let (_, command) = parse_command(input).map_err(|_| "Cannot parse command")?;
    match command {
        ParsedCommand::EqualFilter(column, value) => {
            let mut filter = Filter::default();
            filter.add_equalto_filter(frame, value.as_bytes(), column);
            Ok(Slice::FilterSlice(filter))
        }
        _ => unimplemented!("")
    }
}
