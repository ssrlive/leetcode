#![allow(dead_code)]

/*

// 1609. Even Odd Tree
// https://leetcode.com/problems/even-odd-tree/
// https://leetcode.cn/problems/even-odd-tree/
//
// Medium
//
// A binary tree is named Even-Odd if it meets the following conditions:

    The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
    For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
    For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).

Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.

Example 1:

Input: root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
Output: true
Explanation: The node values on each level are:
Level 0: [1]
Level 1: [10,4]
Level 2: [3,7,9]
Level 3: [12,8,6,2]
Since levels 0 and 2 are all odd and increasing and levels 1 and 3 are all even and decreasing, the tree is Even-Odd.

Example 2:

Input: root = [5,4,2,3,3,7]
Output: false
Explanation: The node values on each level are:
Level 0: [5]
Level 1: [4,2]
Level 2: [3,3,7]
Node values in level 2 must be in strictly increasing order, so the tree is not Even-Odd.

Example 3:

Input: root = [5,9,1,3,5,7]
Output: false
Explanation: Node values in the level 1 should be even integers.

Constraints:

    The number of nodes in the tree is in the range [1, 10^5].
    1 <= Node.val <= 10^6
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;
        if root.is_none() {
            return true;
        }
        let mut prev_level = -1;
        let mut prev = 0;
        let mut que = VecDeque::new();
        que.push_back((0, Rc::clone(&root.unwrap())));
        while let Some((level, node_ref)) = que.pop_front() {
            let node = node_ref.borrow();
            if level == prev_level {
                if level % 2 == 0 {
                    if node.val <= prev {
                        return false;
                    }
                } else if node.val >= prev {
                    return false;
                }
            }
            let carry = if level % 2 == 0 { 0 } else { 1 };
            if node.val % 2 == carry {
                return false;
            }
            if let Some(c) = &node.left {
                que.push_back((level + 1, Rc::clone(c)));
            }
            if let Some(c) = &node.right {
                que.push_back((level + 1, Rc::clone(c)));
            }
            prev_level = level;
            prev = node.val;
        }
        true
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,10,4,3,null,7,9,12,8,6,null,null,2]");
    assert!(Solution::is_even_odd_tree(root));
    let root = TreeNode::from_string("[5,4,2,3,3,7]");
    assert!(!Solution::is_even_odd_tree(root));
    let root = TreeNode::from_string("[5,9,1,3,5,7]");
    assert!(!Solution::is_even_odd_tree(root));
}
