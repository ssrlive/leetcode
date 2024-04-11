#![allow(dead_code)]

// 655. Print Binary Tree
// https://leetcode.com/problems/print-binary-tree/
// https://leetcode.cn/problems/print-binary-tree/
//
// Given the root of a binary tree, construct a 0-indexed m x n string matrix res that represents a
// formatted layout of the tree. The formatted layout matrix should be constructed using the following rules:
//
// - The height of the tree is height and the number of rows m should be equal to height + 1.
// - The number of columns n should be equal to 2^height+1 - 1.
// - Place the root node in the middle of the top row (more formally, at location res[0][(n-1)/2]).
// - For each node that has been placed in the matrix at position res[r][c], place its left child
//   at res[r+1][c-2^height-r-1] and its right child at res[r+1][c+2^height-r-1].
// - Continue this process until all the nodes in the tree have been placed.
// - Any empty cells should contain the empty string "".
//
// Return the constructed matrix res.
//
// Example 1:
//
// Input: root = [1,2]
// Output:
// [["","1",""],
//  ["2","",""]]
//
// Example 2:
//
// Input: root = [1,2,3,null,4]
// Output:
// [["","","","1","","",""],
//  ["","2","","","","3",""],
//  ["","","4","","","",""]]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 2^10].
// - -99 <= Node.val <= 99
// - The depth of the tree will be in the range [1, 10].
//
// Follow up: Could you print this tree in an O(n) space complexity with O(n) time complexity?
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Self::height(&root);
        Self::print_tree_internal(&root, height).unwrap_or_else(|| vec![vec![]])
    }

    fn print_tree_internal(root: &Option<Rc<RefCell<TreeNode>>>, height: i32) -> Option<Vec<Vec<String>>> {
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let val = if root.is_none() {
            String::from("")
        } else {
            root.as_ref()?.borrow().val.to_string()
        };

        if height == 1 {
            return Some(vec![vec![val]]);
        }

        let mut result = Vec::new();
        let len = (1 << (height - 1)) - 1;
        let mut this_level = vec!["".to_string(); len * 2 + 1];
        this_level[len] = val;
        result.push(this_level);
        let left = Self::print_tree_internal(&root.as_ref().unwrap_or(&dummy).borrow().left, height - 1)?;
        let right = Self::print_tree_internal(&root.as_ref().unwrap_or(&dummy).borrow().right, height - 1)?;

        for i in 0..left.len() {
            let mut v = Vec::new();
            v.extend_from_slice(&left[i]);
            v.push(String::from(""));
            v.extend_from_slice(&right[i]);
            result.push(v);
        }

        Some(result)
    }

    fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let left = Self::height(&root.borrow().left);
                let right = Self::height(&root.borrow().right);
                left.max(right) + 1
            }
        }
    }
}

#[test]
fn test() {
    let expected = [vec!["", "1", ""], vec!["2", "", ""]];
    let expected: Vec<Vec<String>> = expected.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let root = TreeNode::from_vec(&[Some(1), Some(2)]);
    assert_eq!(Solution::print_tree(root), expected);

    let expected = [
        vec!["", "", "", "1", "", "", ""],
        vec!["", "2", "", "", "", "3", ""],
        vec!["", "", "4", "", "", "", ""],
    ];
    let expected: Vec<Vec<String>> = expected.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::print_tree(root), expected);

    let expected = [
        vec!["", "", "", "", "", "", "", "1", "", "", "", "", "", "", ""],
        vec!["", "", "", "2", "", "", "", "", "", "", "", "5", "", "", ""],
        vec!["", "3", "", "", "", "", "", "", "", "", "", "", "", "", ""],
        vec!["4", "", "", "", "", "", "", "", "", "", "", "", "", "", ""],
    ];
    let expected: Vec<Vec<String>> = expected.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(5), Some(3), None, None, None, Some(4)]);
    assert_eq!(Solution::print_tree(root), expected);
}
