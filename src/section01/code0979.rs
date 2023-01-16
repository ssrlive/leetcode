#![allow(dead_code)]

// 979. Distribute Coins in Binary Tree
// https://leetcode.com/problems/distribute-coins-in-binary-tree/
// https://leetcode.cn/problems/distribute-coins-in-binary-tree/
//
// You are given the root of a binary tree with n nodes where each node in the tree has node.val coins. There are n coins in total throughout the whole tree.
//
// In one move, we may choose two adjacent nodes and move one coin from one node to another. A move may be from parent to child, or from child to parent.
//
// Return the minimum number of moves required to make every node have exactly one coin.
//
// Example 1:
//
// Input: root = [3,0,0]
// Output: 2
// Explanation: From the root of the tree, we move one coin to its left child, and one coin to its right child.
//
// Example 2:
//
// Input: root = [0,3,0]
// Output: 3
// Explanation: From the left child of the root, we move two coins to the root [taking two moves]. Then, we move one coin from the root of the tree to the right child.
//
// Constraints:
//
// - The number of nodes in the tree is n.
// - 1 <= n <= 100
// - 0 <= Node.val <= n
// - The sum of all Node.val is n.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            match root {
                Some(node) => {
                    let mut node = node.borrow_mut();
                    let left = dfs(node.left.take(), ans);
                    let right = dfs(node.right.take(), ans);
                    *ans += left.abs() + right.abs();
                    node.val + left + right - 1
                }
                None => 0,
            }
        }
        dfs(root, &mut ans);
        ans
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(3), Some(0), Some(0)]);
    assert_eq!(Solution::distribute_coins(root), 2);

    let root = TreeNode::from_vec(&[Some(0), Some(3), Some(0)]);
    assert_eq!(Solution::distribute_coins(root), 3);

    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(2)]);
    assert_eq!(Solution::distribute_coins(root), 2);

    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(0), None, Some(3)]);
    assert_eq!(Solution::distribute_coins(root), 4);
}
