#![allow(dead_code)]

// 938. Range Sum of BST
// https://leetcode.com/problems/range-sum-of-bst/
// https://leetcode.cn/problems/range-sum-of-bst/
//
// Given the root node of a binary search tree and two integers low and high,
// return the sum of values of all nodes with a value in the inclusive range [low, high].
//
// Example 1:
//
// Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
// Output: 32
// Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
//
// Example 2:
//
// Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
// Output: 23
// Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 2 * 10^4].
// - 1 <= Node.val <= 10^5
// - 1 <= low <= high <= 10^5
// - All Node.val are unique.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        pub fn traverse(node: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, sum: &mut i32) {
            if let Some(node) = node {
                let node = node.borrow();

                if (node.val >= low) & (node.val <= high) {
                    *sum += node.val;
                }
                if low < node.val {
                    traverse(node.left.clone(), low, high, sum);
                }
                if node.val < high {
                    traverse(node.right.clone(), low, high, sum);
                }
            }
        }

        let mut sum = 0;
        traverse(root, low, high, &mut sum);
        sum
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(18)]);
    assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);

    let root = TreeNode::from_vec(&[
        Some(10),
        Some(5),
        Some(15),
        Some(3),
        Some(7),
        Some(13),
        Some(18),
        Some(1),
        None,
        Some(6),
    ]);
    assert_eq!(Solution::range_sum_bst(root, 6, 10), 23);
}
