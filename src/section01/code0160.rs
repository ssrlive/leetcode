#![allow(dead_code)]

// 160. Intersection of Two Linked Lists
// https://leetcode.com/problems/intersection-of-two-linked-lists/
// https://leetcode.cn/problems/intersection-of-two-linked-lists/
//
// Given the heads of two singly linked-lists headA and headB, return the node at which the two lists intersect.
// If the two linked lists have no intersection at all, return null.
//
// The test cases are generated such that there are no cycles anywhere in the entire linked structure.
//
// Note that the linked lists must retain their original structure after the function returns.
//
// Custom Judge:
//
// The inputs to the judge are given as follows (your program is not given these inputs):
//
// - intersectVal - The value of the node where the intersection occurs. This is 0 if there is no intersected node.
// - listA - The first linked list.
// - listB - The second linked list.
// - skipA - The number of nodes to skip ahead in listA (starting from the head) to get to the intersected node.
// - skipB - The number of nodes to skip ahead in listB (starting from the head) to get to the intersected node.
//
// The judge will then create the linked structure based on these inputs and pass the two heads, headA and headB to your program. If you correctly return the intersected node, then your solution will be accepted.
//

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
        if v.is_empty() {
            return None;
        }
        let head = Some(Rc::new(RefCell::new(ListNode::new(v[0]))));
        let mut node = head.clone();
        for item in v.iter().skip(1) {
            node.as_ref()?.borrow_mut().next = Some(Rc::new(RefCell::new(ListNode::new(*item))));
            let n = node.as_ref()?.borrow().next.clone();
            node = n;
        }
        head
    }

    pub fn to_vec(head: &Option<Rc<RefCell<ListNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut node = head.clone();
        while let Some(ref n) = node {
            v.push(n.borrow().val);
            let next = n.borrow().next.clone();
            node = next;
        }
        v
    }
}

struct Solution;

impl Solution {
    pub fn get_intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        fn change_sign(head: &Option<Rc<RefCell<ListNode>>>) {
            let mut head = head.clone();
            while let Some(node) = head {
                node.borrow_mut().val *= -1;
                head = node.borrow_mut().next.clone();
            }
        }

        let head_a = head_a;
        change_sign(&head_a);
        let mut head_b = head_b;
        while let Some(node) = head_b {
            if node.borrow().val < 0 {
                head_b = Some(node);
                break;
            }
            head_b = node.borrow().next.clone();
        }
        change_sign(&head_a);
        head_b
    }
}

#[test]
fn test_get_intersection_node() {
    let head_a = ListNode::from_vec(vec![4, 1, 8, 4, 5]);
    let head_b = ListNode::from_vec(vec![5, 0, 1, 8, 4, 5]);
    let head_c = ListNode::from_vec(vec![8, 4, 5]);
    let mut node = head_a.clone();
    while let Some(n) = node {
        if n.borrow().val == 8 {
            node = Some(n);
            break;
        }
        node = n.borrow().next.clone();
    }
    let mut node2 = head_b.clone();
    while let Some(n) = node2 {
        if n.borrow().val == 1 {
            n.borrow_mut().next = node.clone();
            break;
        }
        node2 = n.borrow().next.clone();
    }
    assert_eq!(
        ListNode::to_vec(&Solution::get_intersection_node(head_a, head_b)),
        ListNode::to_vec(&head_c)
    );
}
