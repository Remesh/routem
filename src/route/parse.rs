use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    combinator::recognize,
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::pair,
    Err, IResult,
};

use super::{Param, ParamType, Segment};

pub fn parse_route(input: &str) -> IResult<&str, Vec<Segment>> {
    let param_types = HashMap::new();
    let (input, _) = tag("/")(input)?;
    let (input, segments) = separated_list0(tag("/"), segment(param_types))(input)?;

    Ok((input, segments))
}

fn segment(param_types: &HashMap<&'static str, ParamType>) -> impl Fn(&str) -> IResult<&str, Segment> {
    move |input| {
        alt((param, constant, empty))(input)
    }
}
//fn segment(input: &str) -> IResult<&str, Segment> {
//    alt((param, constant, empty))(input)
//}

fn param(input: &str) -> IResult<&str, Segment> {
    let param_types: HashMap<String, ParamType> = HashMap::new();

    let (input, _) = tag("<")(input)?;

    let (input, name) = identifier(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, kind) = urlsafe_str(input)?;
    let kind = if let Some(param_type) = param_types.get(kind) {
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
    use crate::route::param_type::{self, DEFAULT_PARAM_TYPES};
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

        let (input, output) = segment(&DEFAULT_PARAM_TYPES)(input).unwrap();
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
            kind: custom_type,
        });

        let (input, output) = segment(&DEFAULT_PARAM_TYPES)(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }
}
