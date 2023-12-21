#![allow(dead_code, unused)]

use std::cell::{Ref, RefCell};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Write;
use std::ops::Index;
use std::rc::{Rc, Weak};

use aoc2022::*;

#[derive(Debug, Eq, PartialEq)]
enum TreeEntry {
    Dir(Option<usize>),
    File(usize),
}

struct TreeNode {
    entry: TreeEntry,
    // Empty for files
    children: HashMap<String, SharedMutableTreeNodeRef>,
    parent: Weak<RefCell<TreeNode>>,
}

impl TreeNode {
    fn new(entry: TreeEntry, parent: Weak<RefCell<TreeNode>>) -> TreeNode {
        TreeNode {
            entry,
            children: HashMap::new(),
            parent,
        }
    }

    fn new_shared(entry: TreeEntry, parent: Weak<RefCell<TreeNode>>) -> SharedMutableTreeNodeRef {
        Rc::new(RefCell::new(Self::new(entry, parent)))
    }
}

type SharedMutableTreeNodeRef = Rc<RefCell<TreeNode>>;

trait SharedMutableTreeNodeRefExt {
    fn ensure_child_exists(&self, name: &str, new_entry: TreeEntry) -> Self;
    fn update_dir_size(&self) -> usize;
    fn collect_dir_size(&self, sizes: &mut Vec<usize>);
    fn fmt_debug(&self, ident: usize, name: &str, cwd_ptr: *const TreeNode) -> String;
}

impl SharedMutableTreeNodeRefExt for SharedMutableTreeNodeRef {
    fn ensure_child_exists(&self, name: &str, new_entry: TreeEntry) -> Self {
        let mut self_ref = self.borrow_mut();

        // TODO: HashMap entry API

        match self_ref.children.get(name) {
            None => {
                let new = TreeNode::new_shared(new_entry, Rc::downgrade(self));
                self_ref.children.insert(String::from(name), new.clone());
                new
            }
            Some(sref) => {
                assert_eq!(sref.borrow().entry, new_entry);
                sref.clone()
            }
        }
    }

    fn update_dir_size(&self) -> usize {
        let mut self_ref = self.borrow_mut();

        match self_ref.entry {
            TreeEntry::File(size) => size,
            TreeEntry::Dir(_) => {
                let child_size_sum: usize = self_ref.children.values().map(|c| c.update_dir_size()).sum();
                self_ref.entry = TreeEntry::Dir(Some(child_size_sum));
                child_size_sum
            }
        }
    }

    fn collect_dir_size(&self, sizes: &mut Vec<usize>) {
        match self.borrow().entry {
            TreeEntry::Dir(Some(size)) => {
                sizes.push(size);
                for child in self.borrow().children.values() {
                    child.collect_dir_size(sizes);
                }
            }
            TreeEntry::Dir(None) => panic!("Dir sizes not calculated yet"),
            TreeEntry::File(_) => {}
        }
    }

    fn fmt_debug(&self, ident: usize, name: &str, cwd_ptr: *const TreeNode) -> String {
        let self_ref = self.borrow();
        let mut f = String::new();
        let prefix = " ".repeat(ident);

        // Write self
        let prevlen = prefix.len() + name.len() + 1;
        let tab = " ".repeat(50 - prevlen);
        write!(&mut f, "{prefix} {name}{tab}{:?}", self_ref.entry);
        if std::ptr::eq(&*self_ref, cwd_ptr) {
            write!(&mut f, " <- CWD");
        }
        write!(&mut f, "\n");

        // Write children
        let mut sorted_keys: Vec<&String> = self_ref.children.keys().collect();
        sorted_keys.sort_unstable();
        for name in sorted_keys {
            let child_str = self_ref.children[name].fmt_debug(ident + 2, name, cwd_ptr);
            write!(&mut f, "{}", child_str);
        }

        f
    }
}

struct Tree {
    root: SharedMutableTreeNodeRef,
    cwd: SharedMutableTreeNodeRef,
}

impl Tree {
    fn new() -> Tree {
        let root_ref = TreeNode::new_shared(TreeEntry::Dir(None), Weak::new());
        let cwd_ref = root_ref.clone();
        Tree { root: root_ref, cwd: cwd_ref }
    }

    fn cd_root(&mut self) {
        // Set cwd to point to root
        self.cwd = self.root.clone();
    }

    fn cd_subdir(&mut self, name: &str) {
        // get subdir of cwd
        let subdir = self.cwd.ensure_child_exists(name, TreeEntry::Dir(None));
        // set it as cwd
        self.cwd = subdir;
    }

    fn cd_up(&mut self) {
        // Take current cwd
        // Set cwd to point to its parent
        let mut cwd_ref = self.cwd.borrow();
        let parent = cwd_ref.parent.upgrade();
        drop(cwd_ref);

        if let Some(parent) = parent {
            self.cwd = parent;
        }
    }

    fn add_subdir(&mut self, name: &str) {
        // Same as cd_subdir(), but don't change cwd
        self.cwd.ensure_child_exists(name, TreeEntry::Dir(None));
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.cwd.ensure_child_exists(name, TreeEntry::File(size));
    }

    fn fmt_debug(&self) -> String {
        let cwd_ptr = &*self.cwd.borrow();
        self.root.fmt_debug(0, "/", cwd_ptr)
    }

    fn update_dir_size(&self) {
        self.root.update_dir_size();
    }
}

fn main() {
    let lines = read_lines_as_str("input/day7/input.txt").expect("Failed to read input");

    let mut tree = Tree::new();
    let mut last_is_ls = false;

    for line in &lines {
        assert!(line.len() >= 1);

        println!();
        println!("{}", line);

        if line == "$ cd /" {
            tree.cd_root();
            last_is_ls = false;
        } else if line == "$ cd .." {
            tree.cd_up();
            last_is_ls = false;
        } else if line.starts_with("$ cd ") {
            tree.cd_subdir(&line[5..]);
            last_is_ls = false;
        } else if line == "$ ls" {
            last_is_ls = true;
        } else if last_is_ls && line.starts_with("dir ") {
            // Directory entry in directory listing
            let name = &line[4..];
            tree.add_subdir(name);
        } else if last_is_ls && line.chars().next().unwrap().is_ascii_digit() {
            let (sizestr, name) = line.split_once(' ').unwrap();
            let size: usize = sizestr.parse().unwrap();
            tree.add_file(name, size);
        } else {
            panic!("Unexpected line: {} when state is ls={last_is_ls}", line);
        }
        // println!("{}", tree.fmt_debug());
    }

    tree.update_dir_size();
    println!("{}", tree.fmt_debug());

    let mut sizes = Vec::new();
    tree.root.collect_dir_size(&mut sizes);

    println!("Sum of small directories {}", sizes.iter().filter(|&&s| s <= 100000).sum::<usize>());

    const DISK: usize = 70000000;
    const NEED_FREE: usize = 30000000;

    let used = match tree.root.borrow().entry {
        TreeEntry::Dir(disk) => {
            match disk {
                Some(size) => size,
                None => panic!("should run collect_dir_size()"),
            }
        }
        TreeEntry::File(_) => panic!("root is dir"),
    };

    let free = DISK - used;
    let min_delete = NEED_FREE - free;
    println!("Need to delete {min_delete}");

    println!("Min directory to delete {:?}", sizes.iter().filter(|&&s| s >= min_delete).min());

    // for g in &groups {
    //     let sum: i32 = g.iter().sum();
    //     dbg!(sum);
    // }
    //dbg!(max);
}

fn hello(str: &str) {
    println!("Hello {str}")
}
