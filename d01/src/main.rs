fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    println!("day 2: {}", part_2(lines.iter()));
}

fn get_sums<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> Vec<u64> {
    lines
        .map(|line| {
            let line = line.as_ref().trim();

            if line.is_empty() {
                None
            } else {
                Some(line.parse::<u64>().unwrap())
            }
        })
        .fold(Vec::new(), |mut sums, num| {
            if let Some(num) = num {
                if sums.is_empty() {
                    sums.push(num);
                } else {
                    (*sums.last_mut().unwrap()) += num;
                }
            } else {
                sums.push(0);
            }
            sums
        })
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    let sums = get_sums(lines);
    sums.into_iter().max().expect("no max")
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    let mut sums = get_sums(lines);
    sums.sort();
    sums.iter().rev().take(3).sum::<u64>()
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

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT.lines()), 45000)
    }
}
