// use std::collections::HashMap;

// pub fn run() {
//     let file_contents = include_str!("./input.txt");
//     println!("part1: {}", part1(&file_contents));
//     //println!("part2: {}", part2(&file_contents));
// }

// enum FsEntryType {
//   File, Directory
// }

// /**
// A recursive structure that can represent a file system
// */
// #[derive(Debug)]
// struct FsEntry {
//   entry_type: FsEntryType,
//   size: Option<i32>,
//   name: String,
//   parent: Option<&FsEntry>,
//   children: HashMap<String, FsEntry>,
// }

// fn newDir(d_name: &str, d_parent: &FsEntry) -> FsEntry {
//   FsEntry { entry_type: FsEntryType::Directory, parent: &d_parent }
// }

// fn newFile(f_size: i32, f_name: &str, f_parent: &FsEntry) -> FsEntry {
//   FsEntry { entry_type: FsEntryType::File, parent: &f_parent, size: f_size }
// }

// fn addChildFromLine(current: &FsEntry, child_str: &str) -> FsEntry {
//   let mut parts = child_str.split(" ");
//   let part1 = parts.next();
//   let part2 = parts.next();
//   if part1 == "dir" {
//     current.children.insert(part2, newDir(part2, &current));
//   } else {
//     let fileEntry = newFile(part1.parse::<i32>().unwrap(), part2, &current);
//     current.children.insert(part2, &fileEntry);
//     current.size += fileEntry.size;
//     // have to percolate the size up to grandparent folders too though
//   }
// }

// fn cd(current: &FsEntry, dir_name: &str) -> FsEntry {
//   current.children.get(&dir_name)
// }

// fn cdup(current: &FsEntry) -> FsEntry {
//   match current.parent {
//     Some(x) => return x,
//     None    => panic!("tried to cdup beyond the root"),
//   }
// }

// fn part1(file_contents: &str) -> i32 {
//   let root = FsEntry { entry_type: FsEntryType::Directory, name: "/" };
//   let mut dirs = Vec::new();
//   let mut cwd = &root;
//   let cmds = file_contents.trim().split("\n$ ");
//   for cmd_chunk in cmds {
//     let mut parts = cmd_chunk.split(" ");
//     let cmd = parts.next();
//     let arg = parts.next();
//     match (cmd, arg) {
//       (Some("cd"), Some("/")) => cwd = &root,
//       (Some("cd"), Some("..")) => cwd = cdup(&cwd),
//       (Some("cd"), Some(x)) => cwd = cd(&cwd, x),
//       (Some("ls"), None) => cmd_chunk.lines().skip(1).for_each(|line| { addChildFromLine(&cwd, line); }),
//     }
//   }
//   println!("{:?}", root);
//   9999
// }
