fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("day 1: {}", part_1(&input));
    println!("day 2: {}", part_2(&input));
}

#[derive(Debug)]
struct Forest(Vec<Vec<i64>>);

impl Forest {
    fn size(&self) -> i64 {
        self.0.len() as i64
    }

    fn tree(&self, x: i64, y: i64) -> Option<i64> {
        self.0.get(y as usize)?.get(x as usize).cloned()
    }

    fn is_visible(&self, x: i64, y: i64) -> bool {
        if x == 0 || y == 0 || x == (self.size() - 1) || y == (self.size() - 1) {
            return true;
        }

        if (0..x).all(|check_x| self.tree(check_x, y).unwrap() < self.tree(x, y).unwrap()) {
            return true;
        }

        if ((x + 1)..self.size())
            .all(|check_x| self.tree(check_x, y).unwrap() < self.tree(x, y).unwrap())
        {
            return true;
        }

        if (0..y).all(|check_y| self.tree(x, check_y).unwrap() < self.tree(x, y).unwrap()) {
            return true;
        }

        if ((y + 1)..self.size())
            .all(|check_y| self.tree(x, check_y).unwrap() < self.tree(x, y).unwrap())
        {
            return true;
        }

        false
    }

    fn calculate_viewing_distance(
        &self,
        center_value: i64,
        check_x: i64,
        check_y: i64,
        mut step: impl FnMut(i64, i64) -> (i64, i64),
    ) -> i64 {
        let (recurse, return_value) = if let Some(value) = self.tree(check_x, check_y) {
            if value < center_value {
                (true, 1)
            } else {
                (false, 1)
            }
        } else {
            (false, 0)
        };

        if recurse {
            let (new_x, new_y) = step(check_x, check_y);

            return_value + self.calculate_viewing_distance(center_value, new_x, new_y, step)
        } else {
            return_value
        }
    }

    fn viewing_distance(&self, x: i64, y: i64) -> i64 {
        if x == 0 || y == 0 || x == (self.size() - 1) || y == (self.size() - 1) {
            return 0;
        }

        let center_value = self.tree(x, y).unwrap();

        let l = self.calculate_viewing_distance(center_value, x - 1, y, |x, y| (x - 1, y));
        let r = self.calculate_viewing_distance(center_value, x + 1, y, |x, y| (x + 1, y));

        let o = self.calculate_viewing_distance(center_value, x, y - 1, |x, y| (x, y - 1));
        let u = self.calculate_viewing_distance(center_value, x, y + 1, |x, y| (x, y + 1));

        l * r * o * u
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
                        .map(|ch| ch.to_digit(10).expect("unparseable digit") as i64)
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

fn part_2(input: &str) -> i64 {
    let forest = parse_data(input);

    let mut max_viewing_distance = 0;

    for x in 0..forest.size() {
        for y in 0..forest.size() {
            max_viewing_distance = max_viewing_distance.max(forest.viewing_distance(x, y))
        }
    }
    max_viewing_distance
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
    fn test_hidden_inner(x: i64, y: i64, value: i64, expected: bool) {
        let forest = dbg!(parse_data(TEST_INPUT));
        assert_eq!(forest.tree(x, y).unwrap(), value);
        assert_eq!(forest.is_visible(x, y), expected);
    }

    #[test_case(2, 1, 5, 4; "1")]
    #[test_case(2, 3, 5, 8; "2")]
    fn test_viewing_distance(x: i64, y: i64, value: i64, expected: i64) {
        let forest = dbg!(parse_data(TEST_INPUT));
        assert_eq!(forest.tree(x, y).unwrap(), value);
        assert_eq!(forest.viewing_distance(x, y), expected);
    }

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT), 21)
    }

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT), 8)
    }
}
