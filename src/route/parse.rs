use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    combinator::recognize,
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::pair,
    Err, IResult,
};
use thiserror::Error;

use crate::Route;

use super::{param_type::ParamMap, Param, Segment, ParamType};

pub struct Parser {
    param_types: ParamMap,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unexpected input remaining")]
    ExtraInput {
        segments: Vec<Segment>,
        remainder: String,
    },

    #[error("parse error: {0}")]
    Other(String),
}


impl Default for Parser {
    fn default() -> Self {
        Self {
            param_types: crate::route::param_type::DEFAULT_PARAM_TYPES.clone(),
        }
    }
}

impl Parser {
    pub fn new(param_types: ParamMap) -> Self {
        Self { param_types }
    }

    pub fn add_param_type(&mut self, param_type: ParamType) {
        self.param_types.insert(param_type.typename, param_type);
    }

    /// Parse a route from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use routem::{Parser, Route};
    /// let parser = Parser::default();
    /// let route: Route = parser.route("user-route", "/user/<id:int>/").unwrap();
    /// ```
    pub fn route(&self, name: &str, path: &str) -> Result<Route, ParseError> {
        let segments = match self.parse_route(path) {
            Ok(("", segments)) => segments,
            Ok((remainder, segments)) => {
                return Err(ParseError::ExtraInput {
                    segments,
                    remainder: remainder.to_string(),
                })
            }
            Err(e) => return Err(ParseError::Other(e.to_string())),
        };

        Ok(Route {
            name: name.to_string(),
            path: segments,
        })
    }
}


impl Parser {
    fn parse_route<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<Segment>> {
        let (input, _) = tag("/")(input)?;
        let (input, segments) = separated_list0(tag("/"), |i| self.segment(i))(input)?;

        Ok((input, segments))
    }

    fn segment<'a>(&self, input: &'a str) -> IResult<&'a str, Segment> {
        alt((|i| self.param(i), constant, empty))(input)
    }

    fn param<'a>(&self, input: &'a str) -> IResult<&'a str, Segment> {
        let (input, _) = tag("<")(input)?;

        let (input, name) = identifier(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, kind) = urlsafe_str(input)?;
        let kind = if let Some(param_type) = self.param_types.get(kind) {
            param_type.clone()
        } else {
            return Err(make_error(input));
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
}

fn constant(input: &str) -> IResult<&str, Segment> {
    let (input, s) = urlsafe_str(input)?;
    Ok((input, Segment::Constant(s.to_string())))
}

fn urlsafe_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

fn urlsafe_str(input: &str) -> IResult<&str, &str> {
    take_while1(urlsafe_char)(input)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(take_while_m_n(1, 1, is_alpha), urlsafe_str))(input)
}

fn empty(input: &str) -> IResult<&str, Segment> {
    tag("")(input).map(|(input, _)| (input, Segment::Empty))
}

fn make_error(input: &str) -> Err<Error<&str>> {
    Err::Error(Error::new(input, ErrorKind::Tag))
}

#[cfg(test)]
mod tests {
    use crate::route::param_type;
    use param_type::ParamType;

    use super::*;

    #[test]
    fn test_urlsafe_str() {
        let input = "abc123-_";
        let expected = "abc123-_";

        let (input, output) = urlsafe_str(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_segment() {
        let input = "<id:int>";
        let expected = Segment::Param(Param {
            name: "id".to_string(),
            kind: param_type::INT_PARAM,
        });

        let parser = Parser::default();

        let (input, output) = parser.segment(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_custom_type() {
        fn return_true(_: &str) -> bool {
            true
        }

        let input = "<arg:custom_type>";
        let custom_type = ParamType {
            typename: "custom_type",
            check: return_true,
        };
        let expected = Segment::Param(Param {
            name: "arg".to_string(),
            kind: custom_type.clone(),
        });

        let mut parser = Parser::default();
        parser.add_param_type(custom_type);

        let (input, output) = parser.segment(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }
}
