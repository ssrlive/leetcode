#![allow(dead_code)]

// 543. Diameter of Binary Tree
// https://leetcode.com/problems/diameter-of-binary-tree/
// https://leetcode.cn/problems/diameter-of-binary-tree/
//
// Given the root of a binary tree, return the length of the diameter of the tree.
//
// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
//
// The length of a path between two nodes is represented by the number of edges between them.
//
// Example 1:
//
// Input: root = [1,2,3,4,5]
// Output: 3
// Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
//
// Example 2:
//
// Input: root = [1,2]
// Output: 1
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - -100 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Solution::depth(&root, &mut max);
        max
    }

    fn depth(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(node) = root {
            let left = Solution::depth(&node.borrow().left, max);
            let right = Solution::depth(&node.borrow().right, max);
            *max = (*max).max(left + right);
            left.max(right) + 1
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);

    let root = TreeNode::from_vec(&[Some(1), Some(2)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 1);
}
