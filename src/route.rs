use std::backtrace::Backtrace;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric0},
    combinator::recognize,
    multi::separated_list0,
    sequence::pair,
    IResult,
};
use thiserror::Error;

pub struct Route<T> {
    pub name: String,
    pub path: Vec<Segment>,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Empty,
    Constant(String),
    Param(Param),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub kind: ParamType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamType {
    String,
    Int,
    UUID,
}

#[derive(Error, Debug)]
#[error("could not parse route")]
pub struct ParseError {
    message: String,
    backtrace: Backtrace,
}

impl<T> Route<T> {
    pub fn parse(_s: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

pub fn parse_route(input: &str) -> IResult<&str, Vec<Segment>> {
    let (input, _) = tag("/")(input)?;
    separated_list0(tag("/"), segment)(input)
}

fn segment(input: &str) -> IResult<&str, Segment> {
    alt((param, constant, empty))(input)
}

fn param(input: &str) -> IResult<&str, Segment> {
    let (input, _) = tag("<")(input)?;

    let (input, name) = identifier(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, kind) = alt((tag("string"), tag("int"), tag("uuid")))(input)?;
    let kind = match kind {
        "string" => ParamType::String,
        "int" => ParamType::Int,
        "uuid" => ParamType::UUID,
        _ => unreachable!(),
    };

    let (input, _) = tag(">")(input)?;

    Ok((
        input,
        Segment::Param(Param {
            name: name.to_string(),
            kind,
        }),
    ))
}

fn constant(input: &str) -> IResult<&str, Segment> {
    recognize(take_while1(is_path_char))(input)
        .map(|(input, s)| (input, Segment::Constant(s.to_string())))
}

fn is_path_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, alphanumeric0))(input)
}

fn empty(input: &str) -> IResult<&str, Segment> {
    tag("")(input).map(|(input, _)| (input, Segment::Empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_route() {
        let input = "/user/<id:int>/";
        let expected = vec![
            Segment::Constant("user".to_string()),
            Segment::Param(Param {
                name: "id".to_string(),
                kind: ParamType::Int,
            }),
            Segment::Empty,
        ];

        let (input, output) = parse_route(input).expect("should parse");
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }
}
