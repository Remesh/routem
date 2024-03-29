use thiserror::Error;

pub use crate::route::parse::Parser;

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
pub enum CheckError {
    #[error("malformed path: {0}")]
    MalformedPath(String),
}

impl Route {
    /// Check if a path matches this route.
    ///
    /// # Examples
    ///
    /// ```
    /// use routem::{Parser, Route};
    ///
    /// let parser = Parser::default();
    /// let route = parser.route("user-route", "/user/<id:int>/").unwrap();
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

    /// If a path matches the route, returns the matching params. Otherwise,
    /// returns None.
    ///
    /// # Examples
    ///
    /// ```
    /// use routem::{Parser, Route};
    ///
    /// let parser = Parser::default();
    /// let route = parser.route("user-route", "/user/<id:int>/").unwrap();
    ///
    /// assert_eq!(route.parse_params("/user/123/"), Some(vec!["123".to_string()]));
    /// ```
    pub fn parse_params(&self, path: &str) -> Option<Vec<String>> {
        let clean_path: &str = path.strip_prefix('/').unwrap_or(path);
        let parts = clean_path.split('/').collect::<Vec<&str>>();

        if parts.len() != self.path.len() {
            return None;
        }

        let mut params = Vec::new();
        for (part, segment) in parts.iter().zip(self.path.iter()) {
            match segment {
                Segment::Empty => {
                    if !part.is_empty() {
                        return None;
                    }
                }
                Segment::Constant(s) => {
                    if part != s {
                        return None;
                    }
                }
                Segment::Param(_) => {
                    params.push(part.to_string());
                }
            }
        }

        Some(params)
    }

    /// Fills the supplies parameters into the route. Returns None if the
    /// provided params are the incorrect length.
    ///
    /// # Examples
    /// ```
    /// use routem::{Parser, Route};
    ///
    /// let parser = Parser::default();
    ///
    /// let route = parser.route("user-route", "/user/<id:int>/").unwrap();
    /// let params = vec!["123".to_string()];
    /// assert_eq!(route.fill(&params), Some("/user/123/".to_string()));
    ///
    /// let route = parser.route("long-route", "/user/<id:int>/profile/<profile_id:uuid>").unwrap();
    /// let params = vec!["123".to_string(), "abc".to_string()];
    /// assert_eq!(route.fill(&params), Some("/user/123/profile/abc".to_string()));
    ///
    /// let route = parser.route("empty-route", "/").unwrap();
    /// let params = vec![];
    /// assert_eq!(route.fill(&params), Some("/".to_string()));
    ///
    /// let route = parser.route("user-route", "/user/<id:int>/").unwrap();
    /// let params = vec![];
    /// assert_eq!(route.fill(&params), None);
    /// let params = vec!["123".to_string(), "abc".to_string()];
    /// assert_eq!(route.fill(&params), None);
    /// ```
    pub fn fill(&self, params: &[String]) -> Option<String> {
        let mut path = String::new();

        let mut index = 0;

        for segment in self.path.iter() {
            path.push('/');
            match segment {
                Segment::Empty => {}
                Segment::Constant(s) => {
                    path.push_str(s);
                }
                Segment::Param(_) => {
                    if index >= params.len() {
                        return None;
                    }
                    path.push_str(&params[index]);
                    index += 1;
                }
            }
        }
        if index < params.len() {
            return None;
        }

        Some(path)
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
        let parser = Parser::default();

        let route = parser.route(name, input);
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
        let parser = Parser::default();

        let route = parser.route(name, input);
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
        let parser = Parser::default();

        let route = parser.route(name, input);
        assert!(route.is_ok(), "{:#?}", route);
        let route = route.unwrap();

        assert_eq!(route.name, name);
        assert_eq!(route.path, expected);
    }

    #[test]
    fn test_check_simple_route() {
        let parser = Parser::default();
        let route = parser.route("user-route", "/user/<id:int>/").unwrap();

        println!("{}", route.check("/user/123/"));
        assert!(route.check("/user/123/"));
        assert!(!route.check("/user/123"));
        assert!(!route.check("/user/abc/"));
    }
}
