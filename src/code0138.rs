#![allow(dead_code)]

// 138. Copy List with Random Pointer
// https://leetcode.com/problems/copy-list-with-random-pointer/

// Definition for a Node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node {
            next: None,
            random: None,
            val,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<Node>>> {
        if v.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(Node::new(v[0]?))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < v.len() {
            let node = queue.pop_front()?;
            if let Some(val) = v.get(i)? {
                node.as_ref()?.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(*val))));
                queue.push_back(node.as_ref()?.borrow().next.clone());
            }
            i += 1;
        }
        root
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        head.as_ref()?;
        let mut node = head.clone();
        while let Some(n) = node {
            let mut new_node = Node::new(n.borrow().val);
            new_node.next = n.borrow().next.clone();
            n.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            node = n.borrow().next.as_ref().unwrap().borrow().next.clone();
        }
        let mut node = head.clone();
        while let Some(n) = node {
            let r = n.borrow().random.clone();
            if let Some(r) = r {
                n.borrow_mut().next.as_mut().unwrap().borrow_mut().random = r.borrow().next.clone();
            }
            node = n.borrow().next.as_ref().unwrap().borrow().next.clone();
        }
        let mut node = head;
        let new_head = node.as_ref().unwrap().borrow().next.clone();
        while let Some(n) = node {
            let new_node = n.borrow_mut().next.take().unwrap();
            n.borrow_mut().next = new_node.borrow().next.clone();
            let next = new_node.borrow().next.clone();
            if let Some(next) = next {
                new_node.borrow_mut().next = next.borrow().next.clone();
            }
            node = n.borrow().next.clone();
        }
        new_head
    }
}

#[test]
fn test() {
    let head = Node::from_vec(vec![Some(7), Some(13), Some(11), Some(10), Some(1)]);
    let mut node = head.clone();
    while let Some(n) = node {
        let b = n.borrow().next.clone();
        if let b @ Some(_) = b {
            n.borrow_mut().random = b;
        }
        node = n.borrow().next.clone();
    }
    let new_head = Solution::copy_random_list(head);
    let mut node = new_head.clone();
    while let Some(n) = node {
        let b = n.borrow().next.clone();
        if let b @ Some(_) = b {
            assert_eq!(n.borrow().random, b);
        }
        node = n.borrow().next.clone();
    }
    while let Some(n) = node {
        let val = n.borrow().val;
        let val2 = match n.borrow().random.clone() {
            Some(r) => r.borrow().val,
            None => -1,
        };
        println!("{:?} {:?}", val, val2);
        node = n.borrow().next.clone();
    }
}
