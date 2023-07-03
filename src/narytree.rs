#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// N-ary Tree

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node { val, children: Vec::new() }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<Node>>> {
        let mut v = std::collections::VecDeque::from(v);
        let v0 = v.pop_front()??;
        let root = Some(Rc::new(RefCell::new(Node::new(v0))));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let none = v.pop_front()?;
        assert!(none.is_none());
        while !v.is_empty() {
            let node = queue.pop_front()?;
            loop {
                if v.is_empty() {
                    break;
                }
                let val = v.pop_front()?;
                if let Some(val) = val {
                    let child = Some(Rc::new(RefCell::new(Node::new(val))));
                    node.as_ref()?.borrow_mut().children.push(child.clone());
                    queue.push_back(child);
                } else {
                    break;
                }
            }
        }
        root
    }
}
