use regex::Regex;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let (stacks, moves) = parse_lines(lines.iter());

    println!("day 1: {:?}", part_1(stacks, moves));
    // println!("day 2: {}", part_2(parse_lines(lines.iter())));
}

type Stack = Vec<char>;

#[derive(Debug, PartialEq)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_lines<T: AsRef<str>>(mut lines: impl Iterator<Item = T>) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks: Vec<Stack> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    let mut first_empty_lines_skipped = false;
    let mut stacks_done = false;

    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines {
        let line = line.as_ref();

        if line.trim().is_empty() {
            if first_empty_lines_skipped {
                // empty line in between, means we're done with stacks, and
                // should do moves
                stacks_done = true;
                continue;
            } else {
                continue;
            }
        }

        first_empty_lines_skipped = true;

        let chars: Vec<_> = line.chars().collect();

        if stacks_done {
            let captures = move_re.captures(line).unwrap();
            moves.push(Move {
                amount: dbg!(captures.get(1).unwrap().as_str()).parse().unwrap(),
                from: captures.get(2).unwrap().as_str().parse().unwrap(),
                to: captures.get(3).unwrap().as_str().parse().unwrap(),
            })
        } else {
            for stack in 0..10 {
                let idx = 1 + (stack * 4);
                if chars.len() < idx - 1 {
                    break;
                }

                if chars[idx - 1] == '[' && chars[idx + 1] == ']' {
                    while stacks.len() < stack + 1 {
                        stacks.push(Vec::new());
                    }

                    if stacks[stack].is_empty() {
                        stacks[stack].push(chars[idx]);
                    } else {
                        stacks[stack].insert(0, chars[idx]);
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

// fn part_2(rucksacks: impl Iterator<Item = Rucksack>) -> u64 {
//     rucksacks
//         .tuples::<(_, _, _)>()
//         .map(|(first, second, third)| {
//             let first = first.all_items();
//             let second = second.all_items();
//             let third = third.all_items();

//             let potentially_shared_items: HashSet<Item> =
//                 first.intersection(&second).cloned().collect();
//             let mut really_shared_items = potentially_shared_items.intersection(&third);

//             let result = really_shared_items.next().unwrap();
//             debug_assert!(really_shared_items.next().is_none());

//             result.priority()
//         })
//         .sum::<u64>()
// }

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

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

    // #[test]
    // fn test_2() {
    //     assert_eq!(part_2(parse_lines(TEST_INPUT.lines())), 70)
    // }
}
