use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    println!("day 1: {}", part_1(lines.iter()));
    println!("day 2: {}", part_2(lines.iter()));
}

fn load<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> (usize, Vec<usize>) {
    let mut current_folder: Vec<String> = Vec::new();

    let mut all_file_sizes = 0usize;
    let mut folder_sizes_including_children: HashMap<String, usize> = HashMap::new();

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
                    current_folder.push(folder.into());
                }
            }
        } else if let Some(_dir) = line.strip_prefix("dir ") {
            // nothing for now
        } else if let Some((size, _name)) = line.split_once(' ') {
            let size: usize = size.parse::<usize>().expect("could not parse number");
            all_file_sizes += size;

            for i in 0..current_folder.len() {
                let f = current_folder[0..i + 1].join("");

                folder_sizes_including_children
                    .entry(f)
                    .and_modify(|v| *v += size)
                    .or_insert(size);
            }
        } else {
            unreachable!("unparseable line: {}", line);
        }
    }

    (
        all_file_sizes,
        folder_sizes_including_children.values().copied().collect(),
    )
}

fn part_1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    let (_all, folder_sizes_including_children) = load(lines);
    folder_sizes_including_children
        .iter()
        .filter(|&v| *v <= 100000)
        .sum::<usize>()
}

fn part_2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    let disk_space = 70000000;
    let needed_space = 30000000;

    let (all_file_sizes, folder_sizes_including_children) = load(lines);

    folder_sizes_including_children
        .iter()
        .filter(|size| (disk_space - all_file_sizes + *size) > needed_space)
        .min()
        .copied()
        .expect("no minimum found")
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

    #[test]
    fn test_2() {
        assert_eq!(part_2(TEST_INPUT.lines()), 24933642)
    }
}
