#![allow(dead_code)]

// 1206. Design Skiplist
// https://leetcode.com/problems/design-skiplist/
// https://leetcode.cn/problems/design-skiplist/
//
// Hard
//
// Design a Skiplist without using any built-in libraries.
//
// A skiplist is a data structure that takes O(log(n)) time to add, erase and search. Comparing with treap and red-black tree which has the same function and performance, the code length of Skiplist can be comparatively short and the idea behind Skiplists is just simple linked lists.
//
// For example, we have a Skiplist containing [30,40,50,60,70,90] and we want to add 80 and 45 into it. The Skiplist works this way:
//
// Artyom Kalinin [CC BY-SA 3.0], via Wikimedia Commons
//
// You can see there are many layers in the Skiplist. Each layer is a sorted linked list. With the help of the top layers, add, erase and search can be faster than O(n). It can be proven that the average time complexity for each operation is O(log(n)) and space complexity is O(n).
//
// See more about Skiplist: https://en.wikipedia.org/wiki/Skip_list
//
// Implement the Skiplist class:
//
//     Skiplist() Initializes the object of the skiplist.
//     bool search(int target) Returns true if the integer target exists in the Skiplist or false otherwise.
//     void add(int num) Inserts the value num into the SkipList.
//     bool erase(int num) Removes the value num from the Skiplist and returns true. If num does not exist in the Skiplist, do nothing and return false. If there exist multiple num values, removing any one of them is fine.
//
// Note that duplicates may exist in the Skiplist, your code needs to handle this situation.
//
// Example 1:
//
// Input
// ["Skiplist", "add", "add", "add", "search", "add", "search", "erase", "erase", "search"]
// [[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]
// Output
// [null, null, null, null, false, null, true, false, true, false]
//
// Explanation
// Skiplist skiplist = new Skiplist();
// skiplist.add(1);
// skiplist.add(2);
// skiplist.add(3);
// skiplist.search(0); // return False
// skiplist.add(4);
// skiplist.search(1); // return True
// skiplist.erase(0);  // return False, 0 is not in skiplist.
// skiplist.erase(1);  // return True
// skiplist.search(1); // return False, 1 has already been erased.
//

use rand::Rng;
use std::{cell::RefCell, rc::Rc, vec};

const MAX_LEVEL: usize = 32;
const P_FACTOR: f64 = 0.25;

type Link = Option<Rc<RefCell<Node>>>;

fn new_link(value: i32, level: usize) -> Link {
    Some(Rc::new(RefCell::new(Node::new(value, level))))
}

fn random_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while level < MAX_LEVEL && rng.gen::<f64>() < P_FACTOR {
        level += 1;
    }
    level
}

#[derive(Debug, Default)]
pub struct Node {
    val: i32,
    level: usize,
    forward: Vec<Link>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.forward.len() == other.forward.len()
    }
}

impl Node {
    pub fn new(value: i32, level: usize) -> Node {
        Node {
            val: value,
            level,
            forward: vec![None; level],
        }
    }
}

#[derive(Debug, Default)]
pub struct Skiplist {
    head: Link,
    level: usize,
}

impl Skiplist {
    pub fn new() -> Self {
        Skiplist {
            head: new_link(-1, MAX_LEVEL),
            ..Default::default()
        }
    }

    pub fn search(&self, target: i32) -> bool {
        self._search(target).unwrap_or(false)
    }
    fn _search(&self, target: i32) -> Option<bool> {
        let mut cur = self.head.clone();
        for i in (0..self.level).rev() {
            loop {
                let node = cur.as_ref()?.borrow().forward[i].clone();
                match node {
                    node @ Some(_) => {
                        if node.as_ref()?.borrow().val < target {
                            cur = node;
                            continue;
                        }
                        break;
                    }
                    None => break,
                }
            }
        }
        let node = cur.as_ref()?.borrow().forward[0].clone();
        match node {
            Some(node) => {
                if node.borrow().val == target {
                    return Some(true);
                }
                Some(false)
            }
            None => Some(false),
        }
    }

    pub fn add(&mut self, num: i32) {
        let mut cur = self.head.clone().unwrap();
        let mut update = vec![Some(cur.clone()); MAX_LEVEL];
        for i in (0..self.level).rev() {
            while let Some(node) = cur.clone().borrow().forward[i].clone() {
                if node.borrow().val < num {
                    cur = node;
                    continue;
                }
                break;
            }
            update[i] = Some(cur.clone());
        }
        let level = random_level();
        self.level = self.level.max(level);
        let node = new_link(num, level);
        for (i, item) in update.iter().enumerate().take(level) {
            node.as_ref().unwrap().borrow_mut().forward[i] = item.as_ref().unwrap().borrow().forward[i].clone();
            item.as_ref().unwrap().borrow_mut().forward[i] = node.clone();
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        self._erase(num).unwrap_or(false)
    }
    fn _erase(&mut self, num: i32) -> Option<bool> {
        let mut cur = self.head.clone();
        let mut update: Vec<Link> = vec![None; MAX_LEVEL];
        for i in (0..self.level).rev() {
            loop {
                let node = cur.as_ref()?.borrow().forward[i].clone();
                match node {
                    node @ Some(_) => {
                        if node.as_ref()?.borrow().val < num {
                            cur = node;
                            continue;
                        }
                        break;
                    }
                    None => break,
                }
            }
            update[i] = cur.clone();
        }
        let cur = cur.as_ref()?.borrow().forward[0].clone();
        match cur.clone() {
            Some(node) => {
                if node.borrow().val != num {
                    return Some(false);
                }
            }
            None => return Some(false),
        };

        for (i, item) in update.iter().enumerate().take(self.level) {
            if item.clone().unwrap().borrow().forward[i] != cur.clone() {
                break;
            }
            item.clone().unwrap().borrow_mut().forward[i] = cur.clone().unwrap().borrow().forward[i].clone();
        }
        while self.level > 1 && self.head.clone().unwrap().borrow().forward[self.level - 1].is_none() {
            self.level -= 1;
        }

        Some(true)
    }

    pub fn debug(&self) {
        let head = self.head.clone().unwrap();
        let mut table: Vec<Vec<(i32, usize, usize)>> =
            vec![vec![(head.borrow().val, head.borrow().level, head.borrow().forward.len())]; self.level];

        for i in (0..self.level).rev() {
            let mut node = head.clone();
            while let Some(tmp) = node.clone().borrow().forward[i].clone() {
                table[i].push((tmp.borrow().val, tmp.borrow().level, tmp.borrow().forward.len()));
                node = tmp;
            }
        }
        for t in table.iter().rev() {
            println!("{:?}", t);
        }
    }
}

#[test]
fn test() {
    const N: i32 = 10;
    let mut rng = rand::thread_rng();
    let mut skiplist = Skiplist::new();
    let mut v = vec![];
    for _ in 0..N {
        let num = rng.gen_range(0..N);
        v.push(num);
        skiplist.add(num);
    }
    skiplist.debug();

    for _ in 0..N {
        let num = rng.gen_range(0..N);
        assert_eq!(v.contains(&num), skiplist.search(num));
    }
    for _ in 0..N {
        let num = rng.gen_range(0..N);
        println!("=====");
        println!("{}", num);
        match v.iter().position(|&x| x == num) {
            Some(index) => {
                assert!(skiplist.erase(num));
                v.remove(index);
            }
            None => assert!(!skiplist.erase(num)),
        }
        skiplist.debug()
    }
}
