use std::cmp::Ordering;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    println!("day 2: {}", part_2(lines.iter()));
}

#[derive(PartialEq, Debug)]
enum RPC {
    Rock,
    Paper,
    Scissors,
}

impl RPC {
    fn points(&self) -> u64 {
        match self {
            RPC::Rock => 1,
            RPC::Paper => 2,
            RPC::Scissors => 3,
        }
    }
}

impl PartialOrd for RPC {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (RPC::Rock, RPC::Scissors) => Some(Ordering::Greater),
            (RPC::Scissors, RPC::Rock) => Some(Ordering::Less),
            (RPC::Scissors, RPC::Paper) => Some(Ordering::Greater),
            (RPC::Paper, RPC::Scissors) => Some(Ordering::Less),
            (RPC::Paper, RPC::Rock) => Some(Ordering::Greater),
            (RPC::Rock, RPC::Paper) => Some(Ordering::Less),
            _ => unreachable!("unknown winner, got {:?} {:?}", self, other),
        }
    }
}

impl From<char> for RPC {
    fn from(s: char) -> Self {
        match s {
            'A' => RPC::Rock,
            'B' => RPC::Paper,
            'C' => RPC::Scissors,
            'X' => RPC::Rock,
            'Y' => RPC::Paper,
            'Z' => RPC::Scissors,
            _ => unreachable!("unknown input char"),
        }
    }
}

fn split_lines<T: AsRef<str>>(
    lines: impl Iterator<Item = T>,
) -> impl Iterator<Item = (char, char)> {
    lines.filter_map(|line| {
        let line = line.as_ref().trim();
        if line.is_empty() {
            return None;
        }
        let (lhs, rhs) = line.split_once(' ').expect("could not split");

        Some((lhs.chars().next().unwrap(), rhs.chars().next().unwrap()))
    })
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    split_lines(lines)
        .map(|(other_char, my_char)| {
            let other_move = RPC::from(other_char);
            let my_move = RPC::from(my_char);

            let winning_points = if my_move == other_move {
                3
            } else if my_move > other_move {
                6
            } else {
                0
            };

            winning_points + my_move.points()
        })
        .sum::<u64>()
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    split_lines(lines)
        .map(|(other_char, wanted_result)| {
            let other_move = RPC::from(other_char);

            match wanted_result {
                'X' => {
                    // lose
                    match other_move {
                        RPC::Rock => RPC::Scissors,
                        RPC::Scissors => RPC::Paper,
                        RPC::Paper => RPC::Rock,
                    }
                    .points()
                }
                'Y' => {
                    // draw
                    3 + other_move.points()
                }
                'Z' => {
                    // win
                    6 + match other_move {
                        RPC::Rock => RPC::Paper,
                        RPC::Scissors => RPC::Rock,
                        RPC::Paper => RPC::Scissors,
                    }
                    .points()
                }
                _ => unreachable!("unknown input char"),
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        A Y
        B X
        C Z";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT.lines()), 15)
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT.lines()), 12)
    }
}
