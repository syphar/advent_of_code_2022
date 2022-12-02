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
    fn iter() -> impl Iterator<Item = &'static RPC> {
        static OPTIONS: [RPC; 3] = [RPC::Rock, RPC::Paper, RPC::Scissors];
        OPTIONS.iter()
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl Outcome {
    fn points(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}
impl From<char> for Outcome {
    fn from(s: char) -> Self {
        match s {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!("unknown input char"),
        }
    }
}

fn play(my_move: &RPC, other_move: &RPC) -> Outcome {
    if my_move == other_move {
        Outcome::Draw
    } else {
        match (my_move, other_move) {
            (RPC::Rock, RPC::Scissors) => Outcome::Win,
            (RPC::Scissors, RPC::Rock) => Outcome::Lose,
            (RPC::Scissors, RPC::Paper) => Outcome::Win,
            (RPC::Paper, RPC::Scissors) => Outcome::Lose,
            (RPC::Paper, RPC::Rock) => Outcome::Win,
            (RPC::Rock, RPC::Paper) => Outcome::Lose,
            _ => unreachable!("unknown winner, got {:?} {:?}", my_move, other_move),
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

            play(&my_move, &other_move).points() + my_move.points()
        })
        .sum::<u64>()
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    split_lines(lines)
        .map(|(other_char, wanted_outcome)| {
            let other_move = RPC::from(other_char);

            let wanted_outcome = Outcome::from(wanted_outcome);

            for my_potential_move in RPC::iter() {
                if play(my_potential_move, &other_move) == wanted_outcome {
                    return wanted_outcome.points() + my_potential_move.points();
                }
            }
            unreachable!("no wanted move found");
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
