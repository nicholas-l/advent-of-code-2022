use itertools::Itertools;
use std::{
    cell::RefCell,
    io::BufRead,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum Node_ {
    File {
        name: String,
        size: usize,
        parent: Weak<RefCell<Node_>>,
    },
    Directory {
        name: String,
        children: Vec<Node>,
        parent: Option<Weak<RefCell<Node_>>>,
    },
}

impl Node_ {
    fn get_name(&self) -> String {
        match self {
            Node_::File { name, .. } => name.to_string(),
            Node_::Directory { name, .. } => name.to_string(),
        }
    }
}

type Node = Rc<RefCell<Node_>>;

fn get_directory_size(root: &Node) -> usize {
    match *root.as_ref().borrow() {
        Node_::File { size, .. } => size,
        Node_::Directory { ref children, .. } => children.iter().map(get_directory_size).sum(),
    }
}

fn visit(root: &Node) -> Vec<Node> {
    match *root.as_ref().borrow() {
        Node_::File { .. } => Vec::new(),
        Node_::Directory { ref children, .. } => {
            let mut dirs = vec![root.clone()];
            dirs.extend(children.iter().flat_map(visit));
            dirs
        }
    }
}

fn parse_lines(buf: String) -> Node {
    let mut lines = buf.lines().peekable();
    let root = Rc::new(RefCell::new(Node_::Directory {
        name: "/".to_string(),
        children: Vec::new(),
        parent: None,
    }));

    let mut current_directory = root.clone();

    while let Some(line) = lines.next() {
        match line {
            x if x.starts_with("$ cd") => {
                let new_directory = x.split(' ').last().unwrap();
                match new_directory {
                    ".." => {
                        // Move up one
                        let parent = match &*current_directory.as_ref().borrow() {
                            Node_::File { parent, .. } => parent.clone(),
                            Node_::Directory { parent, .. } => parent.as_ref().cloned().unwrap(),
                        };
                        current_directory = parent.upgrade().unwrap()
                    }
                    "/" => {
                        current_directory = root.clone();
                    }
                    x => match *current_directory.clone().as_ref().borrow() {
                        Node_::File { .. } => todo!(),
                        Node_::Directory { ref children, .. } => {
                            current_directory = children
                                .iter()
                                .find(|c| (*c.as_ref().borrow()).get_name() == x)
                                .cloned()
                                .unwrap_or_else(|| panic!("Tried to find: {}", x))
                        }
                    },
                }
            }
            x if x.starts_with("$ ls") => {
                let new_children =
                    lines
                        .peeking_take_while(|line| !line.starts_with('$'))
                        .map(|x| {
                            if x.starts_with("dir") {
                                let (_type, name) = x.split_once(' ').unwrap();
                                Rc::new(RefCell::new(Node_::Directory {
                                    name: name.to_string(),
                                    children: Vec::new(),
                                    parent: Some(Rc::downgrade(&current_directory)),
                                }))
                            } else {
                                let (size, name) = x.split_once(' ').unwrap();
                                Rc::new(RefCell::new(Node_::File {
                                    name: name.to_string(),
                                    size: size.parse::<usize>().unwrap(),
                                    parent: Rc::downgrade(&current_directory),
                                }))
                            }
                        });
                match *current_directory.borrow_mut() {
                    Node_::File { .. } => panic!("Should not be on a directory"),
                    Node_::Directory {
                        ref mut children, ..
                    } => {
                        children.extend(new_children);
                    }
                }
            }
            x => panic!("{}", x),
        }
    }
    root
}

pub fn star_one(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let root = parse_lines(buf);

    visit(&root)
        .iter()
        .map(|dir| ((*dir).as_ref().borrow().get_name(), get_directory_size(dir)))
        .inspect(|d| println!("{:?}", d))
        .filter(|x| x.1 <= 100_000)
        .map(|x| x.1)
        .sum::<usize>()
        .to_string()
}

pub fn star_two(mut input: impl BufRead) -> String {
    let mut buf = String::new();
    let _res = input.read_to_string(&mut buf);

    let root = parse_lines(buf);

    let mut dir_sizes: Vec<usize> = visit(&root)
        .iter()
        .map(|dir| ((*dir).as_ref().borrow().get_name(), get_directory_size(dir)))
        .map(|x| x.1)
        .collect();

    dir_sizes.sort();

    let space_needed = 30_000_000;
    let space_used = get_directory_size(&root);
    let total_disk = 70000000;

    let to_delete = space_needed - (total_disk - space_used);

    dir_sizes
        .iter()
        .find(|&&x| x > to_delete)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                b"$ cd /
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
7214296 k"
            )),
            "95437"
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(
                b"$ cd /
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
7214296 k"
            )),
            "24933642"
        );
    }
}
