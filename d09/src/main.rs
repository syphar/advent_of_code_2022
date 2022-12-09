fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    println!("day 2: {}", part_2(lines.iter()));
}

#[derive(Debug, Clone)]
struct Pos(i64, i64);

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Move(Direction, i64);

fn parse_lines<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> impl Iterator<Item = Move> {
    lines.filter_map(|line| {
        let line = line.as_ref().trim();
        if line.is_empty() {
            return None;
        }
        let (direction, amount) = line.split_once(' ').expect("could not split");

        Some(Move(
            match direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            },
            amount.parse().expect("could not parse amount"),
        ))
    })
}

fn print_grid(head: &Pos, tail: &Pos) {
    for y in (0..7).rev() {
        let mut line = String::new();
        for x in 0..7 {
            if x == head.0 && y == head.1 {
                line.push('H')
            } else if x == tail.0 && y == tail.1 {
                line.push('T')
            } else {
                line.push('.')
            }
        }
        println!("{}", line);
    }
    println!();
}

fn move_one(el: Pos, mut mov: Move) -> (Pos, Option<Move>) {
    let new_pos = match mov.0 {
        Direction::Left => Pos(el.0.checked_sub(1).unwrap(), el.1),
        Direction::Right => Pos(el.0.checked_add(1).unwrap(), el.1),
        Direction::Up => Pos(el.0, el.1.checked_add(1).unwrap()),
        Direction::Down => Pos(el.0, el.1.checked_sub(1).unwrap()),
    };

    mov.1 = mov.1.checked_sub(1).unwrap();

    (new_pos, if mov.1 == 0 { None } else { Some(mov) })
}

fn is_directly_next_to(pos1: &Pos, pos2: &Pos) -> bool {
    if pos1.0 < 0 || pos1.1 < 0 || pos2.0 < 0 || pos2.1 < 0 {
        return false;
    }

    (pos1.0 == pos2.0 && (((pos1.1 - 1).max(0)..=(pos1.1 + 1)).contains(&pos2.1)))
        || ((pos1.1 == pos2.1) && (((pos1.0 - 1).max(0)..=(pos1.0 + 1)).contains(&pos2.0)))
}

fn is_next_to(pos1: &Pos, pos2: &Pos) -> bool {
    if pos1.0 < 0 || pos1.1 < 0 || pos2.0 < 0 || pos2.1 < 0 {
        return false;
    }

    (((pos1.1 - 1).max(0)..=(pos1.1 + 1)).contains(&pos2.1))
        && (((pos1.0 - 1).max(0)..=(pos1.0 + 1)).contains(&pos2.0))
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> i64 {
    let commands = parse_lines(lines);

    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);

    print_grid(&head, &tail);

    let mut last_command = None;
    for command in commands {
        println!("== {:?} {:?} ==", command.0, command.1);
        let mut command = Some(command);
        let might_be_last_command = command.clone();
        loop {
            if command.is_none() {
                break;
            }
            (head, command) = move_one(head, dbg!(command.unwrap()));

            if !is_next_to(&head, &tail) {
                // try to follow via re-executing the move
                let (try_tail, _) = move_one(tail.clone(), dbg!(last_command.unwrap()));
                if is_directly_next_to(&try_tail, &head) {
                    tail = try_tail;
                } else {
                    let try_upper_left = Pos(tail.0 - 1, tail.1 + 1);
                    if is_next_to(&head, &try_upper_left) {
                        tail = try_upper_left;
                    }
                    let try_upper_right = Pos(tail.0 + 1, tail.1 + 1);
                    if is_next_to(&head, &try_upper_right) {
                        tail = try_upper_right;
                    }
                    let try_lower_left = Pos(tail.0 - 1, tail.1 - 1);
                    if is_next_to(&head, &try_lower_left) {
                        tail = try_lower_left;
                    }
                    let try_lower_right = Pos(tail.0 + 1, tail.1 - 1);
                    if is_next_to(&head, &try_lower_right) {
                        tail = try_lower_right;
                    }
                }
            }
            print_grid(&head, &tail);
        }
        last_command = might_be_last_command;
    }

    todo!();
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT.lines()), 15)
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part_2(TEST_INPUT.lines()), 12)
    // }
}
