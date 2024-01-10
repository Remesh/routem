use thiserror::Error;

use crate::route::parse::parse_route;

pub mod param_type;
pub mod parse;

pub use param_type::ParamType;


#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub name: String,
    pub path: Vec<Segment>,
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

#[derive(Error, Debug)]
pub enum CheckError {
    #[error("malformed path: {0}")]
    MalformedPath(String),
}

impl Route {
    /// Parse a route from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use routem::Route;
    /// let route = Route::parse("user-route", "/user/<id:int>/").unwrap();
    /// ```
    ///
    pub fn parse(name: &str, s: &str) -> Result<Self, ParseError> {
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
        })
    }

    /// Check if a path matches this route.
    ///
    /// # Examples
    ///
    /// ```
    /// use routem::Route;
    ///
    /// let route = Route::parse("user-route", "/user/<id:int>/").unwrap();
    ///
    /// assert!(route.check("/user/123/"));
    /// assert!(!route.check("/user/123"));
    /// assert!(!route.check("/user/abc/"));
    /// ```
    pub fn check(&self, path: &str) -> bool {
        let clean_path: &str = path.strip_prefix('/').unwrap_or(path);
        let parts = clean_path.split('/').collect::<Vec<&str>>();

        if parts.len() != self.path.len() {
            return false;
        }

        for (part, segment) in parts.iter().zip(self.path.iter()) {
            match segment {
                Segment::Empty => {
                    if !part.is_empty() {
                        return false;
                    }
                }
                Segment::Constant(s) => {
                    if part != s {
                        return false;
                    }
                }
                Segment::Param(p) => {
                    if !(p.kind.check)(part) {
                        return false;
                    }
                }
            }
        }


        true
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
                kind: param_type::INT_PARAM,
            }),
            Segment::Empty,
        ];
        let name = "user-route";

        let route = Route::parse(name, input);
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

        let route = Route::parse(name, input);
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
                kind: param_type::INT_PARAM,
            }),
            Segment::Constant("profile".to_string()),
            Segment::Param(Param {
                name: "profile_id".to_string(),
                kind: param_type::UUID_PARAM,
            }),
        ];
        let name = "long-route";

        let route = Route::parse(name, input);
        assert!(route.is_ok(), "{:#?}", route);
        let route = route.unwrap();

        assert_eq!(route.name, name);
        assert_eq!(route.path, expected);
    }

    #[test]
    fn test_check_simple_route() {
        let route = Route::parse("user-route", "/user/<id:int>/").unwrap();

        println!("{}",  route.check("/user/123/"));
        assert!(route.check("/user/123/"));
        assert!(!route.check("/user/123"));
        assert!(!route.check("/user/abc/"));
    }
}
