#![allow(dead_code)]

// 141. Linked List Cycle
// https://leetcode.com/problems/linked-list-cycle/
//
// Given head, the head of a linked list, determine if the linked list has a cycle in it.
//
// There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
//
// Return true if there is a cycle in the linked list. Otherwise, return false.
//

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }

    fn from_vec(v: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
        if v.is_empty() {
            return None;
        }
        let head = Some(Rc::new(RefCell::new(Node::new(v[0]))));
        let mut tail = head.clone();
        for val in v.iter().skip(1) {
            tail.as_ref()?.borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(*val))));
            let t = tail.as_ref()?.borrow().next.clone();
            tail = t;
        }
        head
    }

    fn get_tail(&self) -> Option<Rc<RefCell<Node>>> {
        let mut tail = Some(Rc::new(RefCell::new(self.clone())));
        while tail.as_ref()?.borrow().next.is_some() {
            tail = tail?.borrow().next.clone();
        }
        tail
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<Node>>>) -> bool {
        fn _has_cycle(head: Option<Rc<RefCell<Node>>>) -> Option<bool> {
            let mut slow = head.clone();
            let mut fast = head;
            while fast.is_some() {
                slow = slow?.borrow().next.clone();
                fast = fast?.borrow().next.clone();
                fast = fast?.borrow().next.clone();
                if slow == fast {
                    return Some(true);
                }
            }
            Some(false)
        }
        _has_cycle(head).unwrap_or(false)
    }
}

#[test]
fn test_has_cycle() {
    fn test() -> Option<()> {
        let head = Node::from_vec(vec![3, 2, 0, -4]);
        let tail = head.as_ref()?.borrow().get_tail();
        tail?.borrow_mut().next = head.clone();
        assert_eq!(Solution::has_cycle(head), true);
        Some(())
    }
    assert!(test().is_some());
}
