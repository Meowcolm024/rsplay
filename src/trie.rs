use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

const fn ci(i: u8) -> usize {
    (i - b'a') as usize
}

#[derive(Debug)]
struct Trie {
    root: RefCell<Node>,
}

#[derive(Debug)]
struct Node {
    pub children: Vec<Option<Box<Node>>>,
    pub end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: RefCell::new(Node {
                children: (0..27).map(|_| None).collect(),
                end: false,
            }),
        }
    }

    fn insert(&self, word: String) {
        let mut node = self.root.borrow_mut();
        let mut node = node.deref_mut();
        for i in 0..word.len() {
            if node.children[ci(word.as_bytes()[i])].is_some() {
                let next = node.children[ci(word.as_bytes()[i])].as_mut().unwrap();
                if i == word.len() - 1 {
                    next.end = true;
                }
                node = next;
            } else {
                let next = Node {
                    children: (0..27).map(|_| None).collect(),
                    end: i == word.len() - 1,
                };
                node.children[ci(word.as_bytes()[i])] = Some(Box::new(next));
                node = node.children[ci(word.as_bytes()[i])].as_mut().unwrap();
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let node = self.root.borrow();
        let mut node = node.deref();
        for i in 0..word.len() - 1 {
            if let Some(n) = node.children[ci(word.as_bytes()[i])].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        node.children[ci(word.as_bytes()[word.len() - 1])]
            .as_ref()
            .map_or(false, |x| x.end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        let node = self.root.borrow();
        let mut node = node.deref();
        for i in 0..prefix.len() - 1 {
            if let Some(n) = node.children[ci(prefix.as_bytes()[i])].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        node.children[ci(prefix.as_bytes()[prefix.len() - 1])].is_some()
    }
}

pub fn trie_main() {
    let word = String::from("apple");
    let obj = Trie::new();
    obj.insert(word.clone());
    let r1 = obj.search(word);
    println!("{}", r1);
    let r2 = obj.search("app".to_string());
    println!("{}", r2);
    let r3 = obj.starts_with("app".to_string());
    println!("{}", r3);

    // let ret_2: bool = obj.search(word);
    // let ret_3: bool = obj.starts_with(prefix);
}
