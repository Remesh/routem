use thiserror::Error;

use crate::route::parse::parse_route;

#[derive(Debug, Clone, PartialEq)]
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
pub enum ParseError {
    #[error("unexpected input remaining")]
    ExtraInput { segments: Vec<Segment>, remainder: String },

    #[error("parse error: {0}")]
    Other(String),
}

mod parse;

impl<T> Route<T> {
    pub fn parse(name: &str, s: &str, data: T) -> Result<Self, ParseError> {
        let segments = match parse_route(s) {
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
            data,
        })
    }
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
        let name = "user-route";


        let route = Route::parse(name, input, ());
        assert!(route.is_ok(), "{:#?}", route);
        let route = route.unwrap();

        assert_eq!(route.name, name);
        assert_eq!(route.path, expected);
    }

    #[test]
    fn test_parse_empty_route() {
        let input = "/";
        let expected = vec![Segment::Empty];
        let name = "empty-route";

        let route = Route::parse(name, input, ());
        assert!(route.is_ok(), "{:#?}", route);
        let route = route.unwrap();

        assert_eq!(route.name, name);
        assert_eq!(route.path, expected);

    }

    #[test]
    fn test_parse_long_route() {
        let input = "/user/<id:int>/profile/<profile_id:uuid>";
        let expected = vec![
            Segment::Constant("user".to_string()),
            Segment::Param(Param {
                name: "id".to_string(),
                kind: ParamType::Int,
            }),
            Segment::Constant("profile".to_string()),
            Segment::Param(Param {
                name: "profile_id".to_string(),
                kind: ParamType::UUID,
            }),
        ];
        let name = "long-route";

        let route = Route::parse(name, input, ());
        assert!(route.is_ok(), "{:#?}", route);
        let route = route.unwrap();

        assert_eq!(route.name, name);
        assert_eq!(route.path, expected);
    }
}
