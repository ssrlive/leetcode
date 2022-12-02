#![allow(dead_code)]

// 559. Maximum Depth of N-ary Tree
// https://leetcode.com/problems/maximum-depth-of-n-ary-tree/
//
// Given a n-ary tree, find its maximum depth.
//
// The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
// Nary-Tree input serialization is represented in their level order traversal,
// each group of children is separated by the null value (See examples).
//
// Example 1:
//
// Input: root = [1,null,3,2,4,null,5,6]
// Output: 3
//
// Example 2:
//
// Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
// Output: 5
//
// Constraints:
//
// The total number of nodes is in the range [0, 104].
// The depth of the n-ary tree is less than or equal to 1000.
//

use crate::narytree::Node;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
        if let Some(root) = root {
            let mut max = 0;
            for child in root.borrow().children.iter() {
                max = max.max(Solution::max_depth(child.clone()));
            }
            max + 1
        } else {
            0
        }
    }
}

#[test]
fn test_max_depth() {
    let root = Node::from_vec(vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)]);
    assert_eq!(Solution::max_depth(root), 3);

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
    assert_eq!(Solution::max_depth(root), 5);
}
