#![allow(dead_code)]

// 142. Linked List Cycle II
// https://leetcode.com/problems/linked-list-cycle-ii/
// https://leetcode.cn/problems/linked-list-cycle-ii/
//
// Given a linked list, return the node where the cycle begins. If there is no cycle, return null.
//
// There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer.
// Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.
//
// Notice that you should not modify the linked list.
//
// Follow up:
//
// Can you solve it using O(1) (i.e. constant) memory?

use super::code0141::Node;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn detect_cycle(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        #[allow(clippy::assigning_clones)]
        fn _detect_cycle(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
            let mut slow = head.clone();
            let mut fast = head.clone();
            while fast.is_some() {
                slow = slow?.borrow().next.clone();
                fast = fast?.borrow().next.clone();
                fast = fast?.borrow().next.clone();
                if slow == fast {
                    let mut slow = head;
                    while slow != fast {
                        slow = slow?.borrow().next.clone();
                        fast = fast?.borrow().next.clone();
                    }
                    return slow;
                }
            }
            None
        }
        _detect_cycle(head)
    }
}

#[test]
fn test_detect_cycle() {
    fn test() -> Option<()> {
        let head = Node::from_vec(vec![3, 2, 0, -4]);
        let tail = head.as_ref()?.borrow().get_tail();
        tail?.borrow_mut().next = head.clone();
        let head = Solution::detect_cycle(head);
        assert_eq!(head?.borrow().val, 3);
        Some(())
    }
    assert!(test().is_some());
}
