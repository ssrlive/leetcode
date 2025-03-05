#![allow(dead_code)]

// 1372. Longest ZigZag Path in a Binary Tree
// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/
// https://leetcode.cn/problems/longest-zigzag-path-in-a-binary-tree/
//
// Medium
//
// You are given the root of a binary tree.
//
// A ZigZag path for a binary tree is defined as follow:
//
//     Choose any node in the binary tree and a direction (right or left).
//     If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
//     Change the direction from right to left or from left to right.
//     Repeat the second and third steps until you can't move in the tree.
//
// Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).
//
// Return the longest ZigZag path contained in that tree.
//
// Example 1:
//
// Input: root = [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]
// Output: 3
// Explanation: Longest ZigZag path in blue nodes (right -> left -> right).
//
// Example 2:
//
// Input: root = [1,1,1,null,1,null,null,1,1,null,1]
// Output: 4
// Explanation: Longest ZigZag path in blue nodes (left -> right -> left -> right).
//
// Example 3:
//
// Input: root = [1]
// Output: 0
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 5 * 10^4].
// -    1 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, is_left: bool) -> i32 {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();
                let left = dfs(&node.left, ans, true);
                let right = dfs(&node.right, ans, false);
                *ans = (*ans).max(left).max(right);
                if is_left { right + 1 } else { left + 1 }
            } else {
                0
            }
        }

        let mut ans = 0;
        dfs(&root, &mut ans, true);
        ans
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,1,1,null,1,null,null,1,1,null,1]");
    let ans = 4;
    assert_eq!(Solution::longest_zig_zag(root), ans);

    let root = TreeNode::from_string("[1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]");
    let ans = 3;
    assert_eq!(Solution::longest_zig_zag(root), ans);

    let root = TreeNode::from_string("[1]");
    let ans = 0;
    assert_eq!(Solution::longest_zig_zag(root), ans);
}
