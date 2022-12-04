use std::ops::RangeInclusive;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(parse_lines(lines.iter())));
    println!("day 2: {}", part_2(parse_lines(lines.iter())));
}

type Pair = (RangeInclusive<u64>, RangeInclusive<u64>);

fn parse_lines<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> impl Iterator<Item = Pair> {
    lines.filter_map(|line| {
        let line = line.as_ref().trim();
        if line.is_empty() {
            return None;
        }

        let (lhs, rhs) = line.split_once(',').unwrap();
        let (l1, l2) = lhs.split_once('-').unwrap();
        let (r1, r2) = rhs.split_once('-').unwrap();

        Some((
            RangeInclusive::new(l1.parse().unwrap(), l2.parse().unwrap()),
            RangeInclusive::new(r1.parse().unwrap(), r2.parse().unwrap()),
        ))
    })
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

    static TEST_INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn test_1() {
        assert_eq!(part_1(parse_lines(TEST_INPUT.lines())), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(parse_lines(TEST_INPUT.lines())), 4)
    }
}
