use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, one_of, u64 as parse_u64},
    combinator::{map, opt},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let (stacks, moves) = parse_lines(lines.iter());

    println!("part 1: {:?}", part_1(stacks.clone(), moves.clone()));
    println!("part 2: {:?}", part_2(stacks, moves));
}

type Stack = Vec<char>;

#[derive(Debug, PartialEq, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), parse_u64),
            preceded(tag(" from "), parse_u64),
            delimited(tag(" to "), parse_u64, multispace0),
        )),
        |(amount, from, to)| Move {
            amount: amount as usize,
            from: from as usize,
            to: to as usize,
        },
    )(input)
}
fn parse_stack_element(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(delimited(tag("["), alpha1, tag("]")), |capture: &str| {
            Some(capture.chars().next().unwrap())
        }),
        map(tag("   "), |_| None),
    ))(input)
}

fn parse_stack_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    many1(terminated(parse_stack_element, opt(one_of(" \n\r"))))(input)
}

fn parse_lines<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks: Vec<Stack> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    for line in lines {
        let line = line.as_ref();

        if line.trim().is_empty() {
            continue;
        }

        if let Ok((_, result)) = parse_move(line) {
            moves.push(result);
        }

        if let Ok((_, result)) = parse_stack_line(line) {
            while stacks.len() < result.len() {
                stacks.push(Vec::new());
            }

            for (i, el) in result.into_iter().enumerate() {
                if let Some(ch) = el {
                    if stacks[i].is_empty() {
                        stacks[i].push(ch);
                    } else {
                        stacks[i].insert(0, ch);
                    }
                }
            }
        }
    }
    (stacks, moves)
}

fn part_1(mut stacks: Vec<Stack>, moves: Vec<Move>) -> Vec<char> {
    for Move { amount, from, to } in moves {
        let mut remaining = amount;

        while remaining > 0 {
            let ch = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(ch);
            remaining -= 1;
        }
    }

    stacks
        .iter()
        .map(|stack| stack.iter().rev().next().unwrap())
        .copied()
        .collect()
}

fn part_2(mut stacks: Vec<Stack>, moves: Vec<Move>) -> Vec<char> {
    for Move { amount, from, to } in moves {
        let to_remove = stacks[from - 1].len() - amount..stacks[from - 1].len();
        let mut to_move: Vec<_> = stacks[from - 1].drain(to_remove).collect();
        stacks[to - 1].append(&mut to_move);
    }

    stacks
        .iter()
        .map(|stack| stack.iter().rev().next().unwrap())
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test_case("[D]", Some('D'))]
    #[test_case("   ", None)]
    fn test_parse_stack_element(input: &str, expected: Option<char>) {
        let (remainder, result) = parse_stack_element(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, expected);
    }

    #[test_case("move 1 from 2 to 3", Move {amount: 1, from: 2, to: 3})]
    #[test_case("move 11 from 22 to 33", Move {amount: 11, from: 22, to: 33})]
    fn test_parse_move(input: &str, expected: Move) {
        let (remainder, result) = parse_move(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, expected);
    }

    #[test_case("    [D]    \n", vec![None, Some('D'), None])]
    #[test_case("[N] [C]    \n", vec![Some('N'), Some('C'), None])]
    #[test_case("[Z] [M] [P]\n", vec![Some('Z'), Some('M'), Some('P')])]
    fn test_parse_stack_line(input: &str, expected: Vec<Option<char>>) {
        let (remainder, result) = parse_stack_line(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_input() {
        let (stacks, moves) = parse_lines(TEST_INPUT.lines());

        assert_eq!(
            stacks,
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],]
        );

        assert_eq!(
            moves,
            vec![
                Move {
                    amount: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    amount: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    amount: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    amount: 1,
                    from: 1,
                    to: 2
                },
            ]
        )
    }

    #[test]
    fn test_1() {
        let (stacks, moves) = parse_lines(TEST_INPUT.lines());
        assert_eq!(part_1(stacks, moves), vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_2() {
        let (stacks, moves) = parse_lines(TEST_INPUT.lines());
        assert_eq!(part_2(stacks, moves), vec!['M', 'C', 'D']);
    }
}
