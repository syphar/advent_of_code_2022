use nom::{
    character::complete::multispace0, error::ParseError, multi::many1, sequence::delimited, Parser,
};

pub fn parse_input<'i, O, E, P>(input: &'i str, parser: P) -> Vec<O>
where
    P: Parser<&'i str, O, E>,
    E: ParseError<&'i str> + std::fmt::Debug,
{
    let (remainder, result) =
        many1(delimited(multispace0, parser, multispace0))(input).expect("parse error");
    debug_assert!(remainder.is_empty());
    result
}
