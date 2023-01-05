#![allow(dead_code)]

// 971. Flip Binary Tree To Match Preorder Traversal
// https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal/
// https://leetcode.cn/problems/flip-binary-tree-to-match-preorder-traversal/
//
// You are given the root of a binary tree with n nodes, where each node is uniquely assigned a value from 1 to n.
// You are also given a sequence of n values voyage, which is the desired pre-order traversal of the binary tree.
//
// Any node in the binary tree can be flipped by swapping its left and right subtrees. For example, flipping node 1 will have the following effect:
//
// Flip the smallest number of nodes so that the pre-order traversal of the tree matches voyage.
//
// Return a list of the values of all flipped nodes. You may return the answer in any order.
// If it is impossible to flip the nodes in the tree to make the pre-order traversal match voyage, return the list [-1].
//
// Example 1:
//
// Input: root = [1,2], voyage = [2,1]
// Output: [-1]
// Explanation: It is impossible to flip the nodes such that the pre-order traversal matches voyage.
//
// Example 2:
//
// Input: root = [1,2,3], voyage = [1,3,2]
// Output: [1]
// Explanation: Flipping node 1 swaps nodes 2 and 3, so the pre-order traversal matches voyage.
//
// Example 3:
//
// Input: root = [1,2,3], voyage = [1,2,3]
// Output: []
// Explanation: The tree's pre-order traversal already matches voyage, so no nodes need to be flipped.
//
// Constraints:
//
// - The number of nodes in the tree is n.
// - n == voyage.length
// - 1 <= n <= 100
// - 1 <= Node.val, voyage[i] <= n
// - All the values in the tree are unique.
// - All the values in voyage are unique.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, voyage: &[i32], i: &mut usize, answer: &mut Vec<i32>) -> bool {
            if let Some(n) = node {
                if n.borrow().val != voyage[*i] {
                    return false;
                }
                *i += 1;
                let left = &n.borrow().left;
                if left.is_some() && left.as_ref().map(|l| l.borrow().val) != Some(voyage[*i]) {
                    answer.push(n.borrow().val);
                    dfs(&n.borrow().right, voyage, i, answer) && dfs(&n.borrow().left, voyage, i, answer)
                } else {
                    dfs(&n.borrow().left, voyage, i, answer) && dfs(&n.borrow().right, voyage, i, answer)
                }
            } else {
                true
            }
        }

        let mut i = 0;
        let mut answer = Vec::new();
        if dfs(&root, &voyage, &mut i, &mut answer) {
            answer
        } else {
            vec![-1]
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(2)]);
    let voyage = vec![2, 1];
    let result = Solution::flip_match_voyage(root, voyage);
    assert_eq!(result, vec![-1]);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    let voyage = vec![1, 3, 2];
    let result = Solution::flip_match_voyage(root, voyage);
    assert_eq!(result, vec![1]);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    let voyage = vec![1, 2, 3];
    let result = Solution::flip_match_voyage(root, voyage);
    assert_eq!(result, vec![]);
}
