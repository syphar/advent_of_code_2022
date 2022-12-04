use std::ops::RangeInclusive;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(parse_lines(lines.iter())));
    // println!("day 2: {}", part_2(parse_lines(lines.iter())));
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
    // pairs
    //     .map(|r| {
    //         let mut intersection = r.left.intersection(&r.right);
    //         let shared_item = intersection.next().expect("no shared item");

    //         debug_assert!(intersection.next().is_none());

    //         shared_item.priority()
    //     })
    //     .sum::<u64>()
    pairs
        .filter(|(lhs, rhs)| {
            lhs.clone().into_iter().all(|v| rhs.contains(&v))
                || rhs.clone().into_iter().all(|v| lhs.contains(&v))
        })
        .count() as u64
}

fn part_2(pairs: impl Iterator<Item = Pair>) -> u64 {
    todo!();
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

    // #[test]
    // fn test_2() {
    //     todo!();
    //     // assert_eq!(part_2(parse_lines(TEST_INPUT.lines())), 70)
    // }

    // #[test_case('a', 1 ; "lower case a")]
    // #[test_case('z', 26 ; "lower case z")]
    // #[test_case('A', 27 ; "upper case A")]
    // #[test_case('Z', 52 ; "upper case Z")]
    // fn test_priority(ch: char, prio: u64) {
    //     assert_eq!(Item(ch).priority(), prio);
    // }
}
