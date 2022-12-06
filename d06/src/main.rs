use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("day 1: {}", part_1(&input));
    println!("day 2: {}", part_2(&input));
}

fn find_marker<T: AsRef<str>>(input: T, marker_length: usize) -> usize {
    let chars: Vec<_> = input.as_ref().chars().collect();
    for (i, window) in chars.windows(marker_length).enumerate() {
        if window.iter().duplicates().next().is_none() {
            return i + marker_length;
        }
    }
    unreachable!();
}

fn part_1<T: AsRef<str>>(input: T) -> usize {
    find_marker(input, 4)
}

fn part_2<T: AsRef<str>>(input: T) -> usize {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_1(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_2(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected);
    }
}
