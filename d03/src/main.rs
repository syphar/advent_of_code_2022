use std::collections::HashSet;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(parse_lines(lines.iter())));
    // println!("day 2: {}", part_2(lines.iter()));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn priority(&self) -> u64 {
        match self.0 {
            'a'..='z' => self.0 as u64 - 'a' as u64 + 1,
            'A'..='Z' => self.0 as u64 - 'A' as u64 + 27,
            _ => unreachable!("unexpected character {}", self.0),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    left: HashSet<Item>,
    right: HashSet<Item>,
}

fn parse_lines<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> impl Iterator<Item = Rucksack> {
    lines.filter_map(|line| {
        let line = line.as_ref().trim();
        if line.is_empty() {
            return None;
        }

        debug_assert!(line.len() % 2 == 0);

        let (lhs, rhs) = line.split_at(line.len() / 2);

        debug_assert!(lhs.len() == rhs.len());

        Some(Rucksack {
            left: lhs.chars().map(Item).collect(),
            right: rhs.chars().map(Item).collect(),
        })
    })
}

fn part_1(rucksacks: impl Iterator<Item = Rucksack>) -> u64 {
    rucksacks
        .map(|r| {
            let mut intersection = r.left.intersection(&r.right);
            let shared_item = intersection.next().expect("no shared item");

            debug_assert!(intersection.next().is_none());

            shared_item.priority()
        })
        .sum::<u64>()
}

// fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_1() {
        assert_eq!(part_1(parse_lines(TEST_INPUT.lines())), 157)
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part_2(TEST_INPUT.lines()), 12)
    // }

    #[test_case('a', 1 ; "lower case a")]
    #[test_case('z', 26 ; "lower case z")]
    #[test_case('A', 27 ; "upper case A")]
    #[test_case('Z', 52 ; "upper case Z")]
    fn test_priority(ch: char, prio: u64) {
        assert_eq!(Item(ch).priority(), prio);
    }
}
