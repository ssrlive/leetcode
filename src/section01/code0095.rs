#![allow(dead_code)]

// 95. Unique Binary Search Trees II
// https://leetcode.com/problems/unique-binary-search-trees-ii/
// https://leetcode.cn/problems/unique-binary-search-trees-ii/
//
// Given an integer n, generate all structurally unique BST's (binary search trees) that store values 1...n.
//
// For example,
// Given n = 3, your program should return all 5 unique BST's shown below.
//
//    1         3     3      2      1
//     \       /     /      / \      \
//      3     2     1      1   3      2
//     /     /       \                 \
//    2     1         2                 3

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if l == r {
                return vec![Some(Rc::new(RefCell::new(TreeNode::new(l))))];
            }
            let mut res = vec![];
            for pivot in l..=r {
                let l_trees = if pivot == l { vec![None] } else { helper(l, pivot - 1) };
                let r_trees = if pivot == r { vec![None] } else { helper(pivot + 1, r) };
                for l_item in &l_trees {
                    for r_item in &r_trees {
                        let mut tree = TreeNode::new(pivot);
                        tree.left.clone_from(l_item);
                        tree.right.clone_from(r_item);
                        res.push(Some(Rc::new(RefCell::new(tree))));
                    }
                }
            }
            res
        }

        helper(1, n)
    }
}

#[test]
fn test_generate_trees() {
    let res = Solution::generate_trees(3);
    assert_eq!(res.len(), 5);
}
