#![allow(dead_code)]

// 508. Most Frequent Subtree Sum
// https://leetcode.com/problems/most-frequent-subtree-sum/
// https://leetcode.cn/problems/most-frequent-subtree-sum/
//
// Given the root of a binary tree, return the most frequent subtree sum. If there is a tie, return all the values with the highest frequency in any order.
//
// The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself).
//
// Example 1:
//
// Input: root = [5,2,-3]
// Output: [2,-3,4]
//
// Example 2:
//
// Input: root = [5,2,-5]
// Output: [2]
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - -10^5 <= Node.val <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        Self::dfs(root, &mut map);
        let mut max = 0;
        for (_, v) in map.iter() {
            max = max.max(*v);
        }
        let mut result = Vec::new();
        for (k, v) in map.iter() {
            if *v == max {
                result.push(*k);
            }
        }
        result
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let sum = node.val + Self::dfs(node.left.clone(), map) + Self::dfs(node.right.clone(), map);
            *map.entry(sum).or_insert(0) += 1;
            sum
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(5), Some(2), Some(-3)]);
    let mut result = Solution::find_frequent_tree_sum(root);
    result.sort();
    assert_eq!(result, vec![-3, 2, 4]);
    assert_eq!(
        Solution::find_frequent_tree_sum(TreeNode::from_vec(&[Some(5), Some(2), Some(-5)])),
        vec![2]
    );
}
