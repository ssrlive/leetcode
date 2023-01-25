#![allow(dead_code)]

// 1302. Deepest Leaves Sum
// https://leetcode.com/problems/deepest-leaves-sum/
// https://leetcode.cn/problems/deepest-leaves-sum/
//
// Medium
//
// Given the root of a binary tree, return the sum of values of its deepest leaves.
//
// Example 1:
//
// Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
// Output: 15
//
// Example 2:
//
// Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
// Output: 19
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 10^4].
// -    1 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, sum: &mut i32) {
            if let Some(node) = root {
                let node = node.borrow();
                match depth.cmp(max_depth) {
                    std::cmp::Ordering::Greater => {
                        *max_depth = depth;
                        *sum = node.val;
                    }
                    std::cmp::Ordering::Equal => {
                        *sum += node.val;
                    }
                    _ => {}
                }
                dfs(&node.left, depth + 1, max_depth, sum);
                dfs(&node.right, depth + 1, max_depth, sum);
            }
        }

        let mut max_depth = 0;
        let mut sum = 0;
        dfs(&root, 0, &mut max_depth, &mut sum);
        sum
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(6),
        Some(7),
        None,
        None,
        None,
        None,
        Some(8),
    ]);
    assert_eq!(Solution::deepest_leaves_sum(root), 15);
    let root = TreeNode::from_vec(&[
        Some(6),
        Some(7),
        Some(8),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(9),
        None,
        Some(1),
        Some(4),
        None,
        None,
        None,
        Some(5),
    ]);
    assert_eq!(Solution::deepest_leaves_sum(root), 19);
}
