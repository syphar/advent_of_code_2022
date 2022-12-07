use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    // println!("day 2: {}", part_2(lines.iter()));
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    let mut current_folder: Vec<String> = Vec::new();

    let mut sizes: HashMap<String, usize> = HashMap::new();
    // let mut files: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in lines {
        let line = line.as_ref().trim();
        if line.is_empty() {
            continue;
        }

        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(folder) = command.strip_prefix("cd ") {
                if folder == ".." {
                    current_folder.pop();
                } else if folder == "/" {
                    current_folder.push("/".into());
                } else {
                    current_folder.push(format!("{}/", folder));
                }
            }
        } else if let Some(_dir) = line.strip_prefix("dir ") {
            // nothing for now
        } else if let Some((size, _name)) = line.split_once(' ') {
            let size: usize = size.parse::<usize>().expect("could not parse number");

            // let current_folder = dbg!(current_folder.join(""));

            for i in 0..current_folder.len() {
                let f = current_folder[0..i + 1].join("");

                if let Some(v) = sizes.get_mut(&f) {
                    *v += size;
                } else {
                    sizes.insert(f, size);
                }
            }

            // files
            //     .entry(current_folder)
            //     .or_insert_with(HashMap::new)
            //     .insert(name.to_string(), size);
        } else {
            unreachable!("unparseable line: {}", line);
        }
    }

    sizes.values().filter(|v| v <= &&100000).sum::<usize>()
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

    #[test]
    fn test_1() {
        assert_eq!(part_1(TEST_INPUT.lines()), 95437)
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part_2(TEST_INPUT.lines()), 12)
    // }
}
