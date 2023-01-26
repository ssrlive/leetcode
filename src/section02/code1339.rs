#![allow(dead_code)]

// 1339. Maximum Product of Splitted Binary Tree
// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
// https://leetcode.cn/problems/maximum-product-of-splitted-binary-tree/
//
// Medium
//
// Given the root of a binary tree, split the binary tree into two subtrees by removing one edge
// such that the product of the sums of the subtrees is maximized.
//
// Return the maximum product of the sums of the two subtrees. Since the answer may be too large, return it modulo 109 + 7.
//
// Note that you need to maximize the answer before taking the mod and not after taking it.
//
// Example 1:
//
// Input: root = [1,2,3,4,5,6]
// Output: 110
// Explanation: Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
//
// Example 2:
//
// Input: root = [1,null,2,3,4,null,null,5,6]
// Output: 90
// Explanation: Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [2, 5 * 10^4].
// -    1 <= Node.val <= 10^4
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
            if let Some(n) = node {
                let sum = n.borrow().val as i64 + dfs(&n.borrow().left, sums) + dfs(&n.borrow().right, sums);
                sums.push(sum);
                sum
            } else {
                0
            }
        }

        const MOD: i64 = 1_000_000_007;
        let mut sums = Vec::new();
        dfs(&root, &mut sums);
        ((0..sums.len() - 1)
            .map(|i| sums[i] * (sums[sums.len() - 1] - sums[i]))
            .max()
            .unwrap()
            % MOD) as i32
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,2,3,4,5,6]");
    assert_eq!(Solution::max_product(root), 110);

    let root = TreeNode::from_string("[1,null,2,3,4,null,null,5,6]");
    assert_eq!(Solution::max_product(root), 90);
}
