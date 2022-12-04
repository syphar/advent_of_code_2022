use nom::{
    bytes::complete::tag, character::complete::u64, combinator::map, sequence::separated_pair,
    IResult, Parser,
};
use shared::parsers::parse_input;
use std::ops::RangeInclusive;

fn main() {
    let input = parse_input(
        &std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(),
        parse_pair,
    );

    println!("day 1: {}", part_1(input.iter().cloned()));
    println!("day 2: {}", part_2(input.iter().cloned()));
}

type Pair = (RangeInclusive<u64>, RangeInclusive<u64>);

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(separated_pair(u64, tag("-"), u64), |(start, end)| {
        RangeInclusive::new(start, end)
    })
    .parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn part_1(pairs: impl Iterator<Item = Pair>) -> u64 {
    pairs
        .filter(|(lhs, rhs)| {
            lhs.clone().into_iter().all(|v| rhs.contains(&v))
                || rhs.clone().into_iter().all(|v| lhs.contains(&v))
        })
        .count() as u64
}

fn part_2(pairs: impl Iterator<Item = Pair>) -> u64 {
    pairs
        .filter(|(lhs, rhs)| {
            lhs.clone().into_iter().any(|v| rhs.contains(&v))
                || rhs.clone().into_iter().any(|v| lhs.contains(&v))
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn test_1() {
        assert_eq!(part_1(parse_input(TEST_INPUT, parse_pair).into_iter()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(parse_input(TEST_INPUT, parse_pair).into_iter()), 4)
    }

    #[test_case("123-321", 123..=321)]
    #[test_case("0-1", 0..=1)]
    fn test_parse_range(input: &str, output: RangeInclusive<u64>) {
        let (_, res) = parse_range(input).unwrap();
        assert_eq!(res, output);
    }

    #[test]
    fn test_parse_pair() {
        let (_, res) = parse_pair("1-2,3-4").unwrap();
        assert_eq!(res, (1..=2, 3..=4));
    }

    #[test_case("123-"; "1")]
    #[test_case(""; "2")]
    #[test_case("-"; "3")]
    fn test_parse_range_err(input: &str) {
        assert!(parse_range(input).is_err());
    }
}
