#![allow(dead_code)]

// 429. N-ary Tree Level Order Traversal
// https://leetcode.com/problems/n-ary-tree-level-order-traversal/
//
// Given an n-ary tree, return the level order traversal of its nodes' values.
//
// Nary-Tree input serialization is represented in their level order traversal,
// each group of children is separated by the null value (See examples).
//
// Example 1:
// Input: root = [1,null,3,2,4,null,5,6]
// Output: [[1],[3,2,4],[5,6]]
//
// Example 2:
// Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
// Output: [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]
//
// Constraints:
// - The height of the n-ary tree is less than or equal to 1000
// - The total number of nodes is between [0, 10^4]
//

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
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

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::recursive(root, 0, &mut res);
        res
    }

    fn recursive(root: Option<Rc<RefCell<Node>>>, level: usize, res: &mut Vec<Vec<i32>>) -> Option<()> {
        if root.is_none() {
            return Some(());
        }
        for child in root.as_ref()?.borrow().children.iter() {
            Self::recursive(child.clone(), level + 1, res);
        }
        while res.len() <= level {
            res.push(vec![]);
        }
        res[level].push(root.as_ref()?.borrow().val);
        Some(())
    }
}

#[test]
fn test() {
    let root = Node::from_vec(vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)]);
    let res = vec![vec![1], vec![3, 2, 4], vec![5, 6]];
    assert_eq!(Solution::level_order(root), res);

    let root = Node::from_vec(vec![
        Some(1),
        None,
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        None,
        Some(6),
        Some(7),
        None,
        Some(8),
        None,
        Some(9),
        Some(10),
        None,
        None,
        Some(11),
        None,
        Some(12),
        None,
        Some(13),
        None,
        None,
        Some(14),
    ]);
    let res = vec![
        vec![1],
        vec![2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13],
        vec![14],
    ];
    assert_eq!(Solution::level_order(root), res);
}
