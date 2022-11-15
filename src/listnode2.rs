#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: &[i32]) -> Option<Rc<RefCell<ListNode>>> {
        let mut head = None;
        let mut tail: Option<Rc<RefCell<ListNode>>> = None;
        for val in v.iter() {
            let node = Rc::new(RefCell::new(ListNode::new(*val)));
            if let Some(tail) = tail {
                tail.borrow_mut().next = Some(node.clone());
            } else {
                head = Some(node.clone());
            }
            tail = Some(node);
        }
        head
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = vec![];
        let mut node = Some(Rc::new(RefCell::new(self.clone())));
        while let Some(n) = node {
            v.push(n.borrow().val);
            node = n.borrow().next.clone();
        }
        v
    }

    pub fn find_node(root: Option<Rc<RefCell<ListNode>>>, val: i32) -> Option<Rc<RefCell<ListNode>>> {
        let mut node = root;
        while let Some(n) = node {
            if n.borrow().val == val {
                return Some(n);
            }
            node = n.borrow().next.clone();
        }
        None
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_vec();
        write!(f, "{:?}", v)
    }
}
