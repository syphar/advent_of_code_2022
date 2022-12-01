fn main() {
    let lines: Vec<String> = std::fs::read_to_string(dbg!(std::env::args().nth(1).unwrap()))
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()))
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    let mut sums: Vec<u64> = Vec::new();

    let mut current_sum: Option<u64> = None;

    for line in lines {
        let line = line.as_ref().trim();

        if line.is_empty() {
            if let Some(s) = current_sum {
                sums.push(s);
                current_sum = None;
            } else {
                continue;
            }
        } else {
            let value: u64 = line.parse().expect("could not parse number");

            if let Some(s) = current_sum {
                current_sum = Some(value + s);
            } else {
                current_sum = Some(value);
            }
        }
    }
    if let Some(s) = current_sum {
        sums.push(s);
    }

    sums.into_iter().max().expect("no max")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT.lines()), 24000)
    }
}
