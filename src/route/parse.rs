use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    combinator::recognize,
    multi::separated_list0,
    sequence::pair,
    IResult,
};

use super::{Param, ParamType, Segment};

pub fn parse_route(input: &str) -> IResult<&str, Vec<Segment>> {
    let (input, _) = tag("/")(input)?;
    let (input, segments) = separated_list0(tag("/"), segment)(input)?;

    Ok((input, segments))
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

#[cfg(test)]
mod tests {
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
            kind: ParamType::Int,
        });

        let (input, output) = segment(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected);
    }
}
