use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    sequence::delimited,
    IResult, Parser,
};

pub fn parse_instruction(input: &str) -> IResult<&str, &str> {
    alt((tag("Filter"), tag("Average")))(input)
}

pub fn parse_filter_column(symbol: &str) -> impl Parser<&str, &str, nom::error::Error<&str>> {
    let column = take_until(symbol);
    delimited(multispace0, column, multispace0)
}

pub fn parse_filter_symbol(input: &str) -> IResult<&str, &str> {
    alt((
        parse_filter_column(" ="),
        parse_filter_column(" <"),
        parse_filter_column(" >"),
    ))(input)
}

pub fn parse_filter(input: &str) -> IResult<&str, ParsedCommand> {
    let err = Err(nom::Err::Error(nom::error::Error::new(
        "Unknown command",
        nom::error::ErrorKind::Tag,
    )));

    let (input, column) = parse_filter_symbol(input)?;
    let (value, symbol) = alt((tag("= "), tag("< "), tag("> ")))(input)?;
    match symbol {
        "= " => Ok(("", ParsedCommand::EqualFilter(column, value))),
        _ => err,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsedCommand<'a> {
    EqualFilter(&'a str, &'a str),
    LessFilter(&'a str, &'a str),
    GreaterFilter(&'a str, &'a str),
    Average,
}

pub fn parse_command(input: &str) -> IResult<&str, ParsedCommand> {
    let (tail, keyword) = parse_instruction(input)?;
    let err = Err(nom::Err::Error(nom::error::Error::new(
        "Unknown command",
        nom::error::ErrorKind::Tag,
    )));

    match keyword {
        "Filter" => {
            let (_, command) = parse_filter(tail)?;
            Ok((keyword, command))
        }
        _ => err,
    }
}

#[cfg(test)]
mod test {
    use super::parse_instruction;
    use crate::command::parser::{parse_filter, ParsedCommand};

    #[test]
    fn command() {
        let (tail, command) = parse_instruction("Filter Type 1 = Fire").unwrap();
        let ret = match command {
            "Filter" => parse_filter(tail).unwrap().1,
            _ => ParsedCommand::Average,
        };
        assert_eq!(ret, ParsedCommand::EqualFilter("Type 1", "Fire"));
    }

    #[test]
    fn err() {
        let res = parse_instruction("NoCommand Type 1 = Fire");
        let err = Err(nom::Err::Error(nom::error::Error::new(
            "NoCommand Type 1 = Fire",
            nom::error::ErrorKind::Tag,
        )));
        assert_eq!(res, err);
    }
}
