use std::ops::Index;

#[derive(Debug)]
struct Item<'a> {
    parent: usize,
    name: &'a str,
    item_type: ItemType,
}

impl<'a> Item<'a> {
    pub fn size(&self) -> i64 {
        match &self.item_type {
            ItemType::File { size } => *size,
            ItemType::Directory { total_size, .. } => *total_size,
        }
    }
}

#[derive(Debug)]
enum ItemType {
    File { size: i64 },
    Directory { total_size: i64, items: Vec<usize> },
}

#[derive(Debug)]
struct Drive<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> Index<(usize, &'a str)> for Drive<'a> {
    type Output = usize;

    fn index(&self, (index, name): (usize, &str)) -> &Self::Output {
        let node = &self.items[index];
        let ItemType::Directory { total_size: _, items } = &node.item_type else { unreachable!()};

        items.iter().find(|i| self.items[**i].name == name).unwrap()
    }
}

impl<'a> Drive<'a> {
    pub fn new() -> Self {
        Drive { items: Vec::new() }
    }

    pub fn add_item(&mut self, folder: usize, item: Item<'a>) {
        let item_index = self.items.len();

        let file_size = if let ItemType::File { size } = &item.item_type {
            Some(*size)
        } else {
            None
        };

        self.items.push(item);

        let mut parent_folder;

        match &mut self.items[folder] {
            Item {
                parent,
                item_type: ItemType::Directory { total_size, items },
                ..
            } => {
                items.push(item_index);

                if let Some(size) = file_size {
                    *total_size += size;
                }

                parent_folder = *parent;
            }
            _ => unreachable!(),
        }

        if folder == 0 {
            return;
        }

        let Some(file_size) = file_size else {
            return;
        };

        loop {
            let Item { parent, item_type: ItemType::Directory { total_size, .. }, ..} = &mut self.items[parent_folder] else { 
                unreachable!()
            };

            *total_size += file_size;

            if parent_folder == 0 {
                return;
            }

            parent_folder = *parent;
        }
    }
}

#[aoc(day7, part1)]
pub fn run_part1(input: &str) -> i64 {
    let drive = load_drive(input);

    drive
        .items
        .into_iter()
        .filter_map(|i| match i.item_type {
            ItemType::Directory {
                total_size: size @ 0..=100_000,
                ..
            } => Some(size),
            _ => None,
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn run_part2(input: &str) -> i64 {
    let drive = load_drive(input);

    const TOTAL_SPACE: i64 = 70_000_000;
    const NEEDED_SPACE: i64 = 30_000_000;

    let used_space = drive.items[0].size();
    let unused_space = TOTAL_SPACE - used_space;
    let space_to_delete = NEEDED_SPACE - unused_space;

    drive
        .items
        .into_iter()
        .filter_map(|i| match i.item_type {
            ItemType::Directory {
                total_size,
                ..
            } if total_size >= space_to_delete => Some(total_size),
            _ => None,
        })
        .min()
        .unwrap()
}

fn load_drive(input: &str) -> Drive<'_> {
    let root = Item {
        parent: 0,
        name: "/",
        item_type: ItemType::Directory {
            total_size: 0,
            items: Vec::new(),
        },
    };

    let mut drive = Drive::new();
    let mut cwd = 0;

    drive.items.push(root);

    let mut ls_mode = false;

    for line in input.split('\n') {
        if ls_mode && &line[..1] == "$" {
            ls_mode = false;
        }

        if ls_mode {
            let (dir_or_size, name) = line.split_once(' ').unwrap();

            let item = if dir_or_size == "dir" {
                Item {
                    parent: cwd,
                    name,
                    item_type: ItemType::Directory {
                        total_size: 0,
                        items: Vec::new(),
                    },
                }
            } else {
                Item {
                    parent: cwd,
                    name,
                    item_type: ItemType::File {
                        size: dir_or_size.parse().unwrap(),
                    },
                }
            };

            drive.add_item(cwd, item);
        } else {
            match &line[..4] {
                "$ ls" => {
                    ls_mode = true;
                }
                "$ cd" => {
                    let dir_name = &line[5..];

                    match dir_name {
                        "/" => {
                            cwd = 0;
                        }
                        ".." => {
                            cwd = drive.items[cwd].parent;
                        }
                        _ => {
                            cwd = drive[(cwd, dir_name)];
                        }
                    }
                }
                _ => {}
            }
        }
    }

    drive
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "$ cd /
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
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 95437);
    }

    #[test]
    fn part2_naive_example() {
        assert_eq!(run_part2(TEST_INPUT), 24933642);
    }
}
