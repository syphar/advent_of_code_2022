fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("day 1: {}", part_1(&input));
    // println!("day 2: {}", part_2(&input));
}

#[derive(Debug)]
struct Forest(Vec<Vec<u32>>);

impl Forest {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn tree(&self, x: usize, y: usize) -> u32 {
        self.0[y][x]
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == (self.size() - 1) || y == (self.size() - 1) {
            return true;
        }

        if (0..x).all(|check_x| self.tree(check_x, y) < self.tree(x, y)) {
            return true;
        }

        if ((x + 1)..self.size()).all(|check_x| self.tree(check_x, y) < self.tree(x, y)) {
            return true;
        }

        if (0..y).all(|check_y| self.tree(x, check_y) < self.tree(x, y)) {
            return true;
        }

        if ((y + 1)..self.size()).all(|check_y| self.tree(x, check_y) < self.tree(x, y)) {
            return true;
        }

        false
    }
}

fn parse_data(input: &str) -> Forest {
    Forest(
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                Some(
                    line.chars()
                        .map(|ch| ch.to_digit(10).expect("unparseable digit"))
                        .collect::<Vec<_>>(),
                )
            })
            .collect(),
    )
}

fn part_1(input: &str) -> u64 {
    let forest = parse_data(input);

    let mut visible_trees = 0;

    for x in 0..forest.size() {
        for y in 0..forest.size() {
            if forest.is_visible(x, y) {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn part_2(input: &str) -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static TEST_INPUT: &str = "
        30373
        25512
        65332
        33549
        35390";

    #[test_case(1, 1, 5, true; "1")]
    #[test_case(2, 1, 5, true; "2")]
    #[test_case(3, 1, 1, false; "3")]
    #[test_case(1, 2, 5, true; "4")]
    #[test_case(1, 2, 5, true; "5")]
    #[test_case(2, 2, 3, false; "6")]
    #[test_case(3, 2, 3, true; "7")]
    #[test_case(1, 3, 3, false; "8")]
    #[test_case(2, 3, 5, true; "9")]
    #[test_case(3, 3, 4, false; "10")]
    fn test_hidden_inner(x: usize, y: usize, value: u32, expected: bool) {
        let forest = dbg!(parse_data(TEST_INPUT));
        assert_eq!(forest.tree(x, y), value);
        assert_eq!(forest.is_visible(x, y), expected);
    }

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT), 21)
    }

    #[test]
    fn test_2() {
        assert_eq!(part_1(TEST_INPUT), 8)
    }
}
