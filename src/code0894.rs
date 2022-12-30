#![allow(dead_code)]

// 894. All Possible Full Binary Trees
// https://leetcode.com/problems/all-possible-full-binary-trees/
// https://leetcode.cn/problems/all-possible-full-binary-trees/
//
// Given an integer n, return a list of all possible full binary trees with n nodes. Each node of each tree in the answer must have Node.val == 0.
//
// Each element of the answer is the root node of one possible tree. You may return the final list of trees in any order.
//
// A full binary tree is a binary tree where each node has exactly 0 or 2 children.
//
// Example 1:
//
// Input: n = 7
// Output: [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
//
// Example 2:
//
// Input: n = 3
// Output: [[0,0,0]]
//
// Constraints:
//
// - 1 <= n <= 20
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];
        if n % 2 == 0 {
            return result;
        }
        if n == 1 {
            result.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
            return result;
        }
        for i in 1..n {
            let left = Self::all_possible_fbt(i);
            let right = Self::all_possible_fbt(n - i - 1);
            for l in left {
                for r in &right {
                    let mut root = TreeNode::new(0);
                    root.left = l.clone();
                    root.right = r.clone();
                    result.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let result = Solution::all_possible_fbt(7);
    result.iter().for_each(|r| {
        println!("{:?}", r.as_ref().unwrap().borrow().to_vec());
    });

    let result = Solution::all_possible_fbt(3);
    result.iter().for_each(|r| {
        println!("{:?}", r.as_ref().unwrap().borrow().to_vec());
    });
}
