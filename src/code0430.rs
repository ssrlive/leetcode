#![allow(dead_code)]

// 430. Flatten a Multilevel Doubly Linked List
// https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/
//
// You are given a doubly linked list, which contains nodes that have a next pointer, a previous pointer,
// and an additional child pointer. This child pointer may or may not point to a separate doubly linked list,
// also containing these special nodes. These child lists may have one or more children of their own, and so on,
// to produce a multilevel data structure as shown in the example below.
//
// Given the head of the first level of the list, flatten the list so that all the nodes appear in a single-level,
// doubly linked list. Let curr be a node with a child list. The nodes in the child list should appear after curr
// and before curr.next in the flattened list.
//
// Return the head of the flattened list. The nodes in the list must have all of their child pointers set to null.
//
// Example 1:
//
// Input: head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
// Output: [1,2,3,7,8,11,12,9,10,4,5,6]
// Explanation: The multilevel linked list in the input is shown.
// After flattening the multilevel linked list it becomes:
//
// Example 2:
//
// Input: head = [1,2,null,3]
// Output: [1,3,2]
// Explanation: The multilevel linked list in the input is shown.
// After flattening the multilevel linked list it becomes:
//
// Example 3:
//
// Input: head = []
// Output: []
// Explanation: There could be empty list in the input.
//
// Constraints:
//
// - The number of Nodes will not exceed 1000.
// - 1 <= Node.val <= 105
//
// How the multilevel linked list is represented in test cases:
//
// We use the multilevel linked list from Example 1 above:
//
//  1---2---3---4---5---6--NULL
//          |
//          7---8---9---10--NULL
//              |
//              11--12--NULL
//
// The serialization of each level is as follows:
//
// [1,2,3,4,5,6,null]
// [7,8,9,10,null]
// [11,12,null]
//
// To serialize all levels together, we will add nulls in each level to signify no node connects
// to the upper node of the previous level. The serialization becomes:
//
// [1,    2,    3, 4, 5, 6, null]
//              |
// [null, null, 7,    8, 9, 10, null]
//                    |
// [            null, 11, 12, null]
//
// Merging the serialization of each level and removing trailing nulls we obtain:
//
// [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
//

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
    pub child: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            prev: None,
            next: None,
            child: None,
            val,
        }
    }

    pub fn from_vec(v: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
        let sec = v.split(|x| x.is_none()).collect::<Vec<_>>();
        let mut sec = sec.iter().map(|&x| Self::_from_vec(x)).collect::<VecDeque<_>>();
        println!("{:?}", sec);

        let mut count = 0;
        let result = sec.pop_front()?;
        let mut parent = result.clone();
        while !sec.is_empty() {
            let child = match sec.pop_front() {
                Some(x) => x,
                None => break,
            };
            if child.is_none() {
                count += 1;
                continue;
            }
            while count > 0 {
                let temp = parent.as_ref()?.borrow_mut().next.clone();
                parent = temp;
                count -= 1;
            }
            parent.as_ref()?.borrow_mut().child = child.clone();
            parent = child;
        }
        result
    }

    fn _from_vec(v: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
        let f = |x| Rc::new(RefCell::new(Node::new(x)));
        let nodes = v.iter().map(|&x| x.map(f)).collect::<Vec<_>>();
        for i in 0..nodes.len() {
            if let Some(ref node) = nodes[i] {
                if i > 0 {
                    node.borrow_mut().prev = Some(Rc::downgrade(nodes[i - 1].as_ref()?));
                }
                if i < nodes.len() - 1 {
                    node.borrow_mut().next = nodes[i + 1].clone();
                }
            }
        }
        if nodes.is_empty() {
            None
        } else {
            nodes[0].clone()
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        if let Some(ref child) = self.child {
            write!(f, "({})", child.borrow())?;
        }
        if let Some(ref next) = self.next {
            write!(f, " -> {}", next.borrow())?;
        }
        Ok(())
    }
}

struct Solution;

impl Solution {
    pub fn flatten(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut head = head;
        let mut dummy = Some(Rc::new(RefCell::new(Node::new(0))));
        Solution::dfs(&mut dummy, &mut head);
        head.as_ref()?.borrow_mut().prev = None;
        head
    }

    fn dfs(ans: &mut Option<Rc<RefCell<Node>>>, cur: &mut Option<Rc<RefCell<Node>>>) -> Option<()> {
        if cur.is_none() {
            return Some(());
        }
        let mut c = cur.as_ref()?.borrow_mut().child.take();
        let mut r = cur.as_ref()?.borrow_mut().next.clone();
        ans.as_ref()?.borrow_mut().next = cur.clone();
        cur.as_ref()?.borrow_mut().prev = Some(Rc::downgrade(&ans.clone().unwrap()));
        ans.as_ref()?.borrow_mut().child = None;
        *ans = cur.clone();
        Solution::dfs(ans, &mut c);
        Solution::dfs(ans, &mut r);
        Some(())
    }
}

#[test]
fn test_flatten() {
    let head = Node::from_vec(&[]);
    assert!(head.is_none());
    let head = Solution::flatten(head);
    assert!(head.is_none());

    let head = Node::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        None,
        None,
        None,
        Some(7),
        Some(8),
        Some(9),
        Some(10),
        None,
        None,
        Some(11),
        Some(12),
    ]);
    println!("{}", head.as_ref().unwrap().borrow());
    let head = Solution::flatten(head);
    let result = Node::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(7),
        Some(8),
        Some(11),
        Some(12),
        Some(9),
        Some(10),
        Some(4),
        Some(5),
        Some(6),
    ]);
    assert_eq!(
        head.as_ref().unwrap().borrow().to_string(),
        result.as_ref().unwrap().borrow().to_string()
    );

    let head = Node::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    let head = Solution::flatten(head);
    let result = Node::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(
        head.as_ref().unwrap().borrow().to_string(),
        result.as_ref().unwrap().borrow().to_string()
    );

    let head = Node::from_vec(&[Some(1), Some(2), None, Some(3)]);
    let head = Solution::flatten(head);
    let result = Node::from_vec(&[Some(1), Some(3), Some(2)]);
    assert_eq!(
        head.as_ref().unwrap().borrow().to_string(),
        result.as_ref().unwrap().borrow().to_string()
    );
}
