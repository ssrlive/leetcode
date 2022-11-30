#![allow(dead_code)]

// 501. Find Mode in Binary Search Tree
// https://leetcode.com/problems/find-mode-in-binary-search-tree/
//
// Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.
//
// If the tree has more than one mode, return them in any order.
//
// Assume a BST is defined as follows:
//
// The left subtree of a node contains only nodes with keys less than or equal to the node's key.
// The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
// Both the left and right subtrees must also be binary search trees.
//
// Example 1:
//
// Input: root = [1,null,2,2]
// Output: [2]
//
// Example 2:
//
// Input: root = [0]
// Output: [0]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 104].
// - -105 <= Node.val <= 105
//
// Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut max_count = 0;
        let mut current_count = 0;
        let mut current_value = 0;
        let mut traverse = |value: i32| {
            if value == current_value {
                current_count += 1;
            } else {
                match current_count.cmp(&max_count) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => result.push(current_value),
                    std::cmp::Ordering::Greater => {
                        result.clear();
                        result.push(current_value);
                        max_count = current_count;
                    }
                }
                current_count = 1;
                current_value = value;
            }
        };
        Solution::traverse(root, &mut traverse);
        match current_count.cmp(&max_count) {
            std::cmp::Ordering::Greater => {
                result.clear();
                result.push(current_value);
            }
            std::cmp::Ordering::Equal => {
                result.push(current_value);
            }
            std::cmp::Ordering::Less => {}
        }
        result
    }
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, traverse: &mut dyn FnMut(i32)) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::traverse(node.left.clone(), traverse);
            traverse(node.val);
            Solution::traverse(node.right.clone(), traverse);
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(2)]);
    let result = Solution::find_mode(root);
    assert_eq!(result, vec![2]);
}
