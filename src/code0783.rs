#![allow(dead_code)]

// 782. Minimum Distance Between BST Nodes
// https://leetcode.com/problems/minimum-distance-between-bst-nodes/
// https://leetcode.cn/problems/minimum-distance-between-bst-nodes/
//
// Given the root of a Binary Search Tree (BST), return the minimum difference between the values of any two different nodes in the tree.
//
// Example 1:
//
// Input: root = [4,2,6,1,3]
// Output: 1
//
// Example 2:
//
// Input: root = [1,0,48,null,null,12,49]
// Output: 1
//
// Constraints:
//
// - The number of nodes in the tree is in the range [2, 100].
// - 0 <= Node.val <= 10^5
//
// Note: This question is the same as 530: https://leetcode.com/problems/minimum-absolute-difference-in-bst/
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, min: &mut i32) {
            if let Some(node) = root {
                let node = node.borrow();
                inorder(node.left.clone(), prev, min);
                if let Some(prev) = prev {
                    *min = (*min).min(node.val - *prev);
                }
                *prev = Some(node.val);
                inorder(node.right.clone(), prev, min);
            }
        }

        let mut min = i32::MAX;
        let mut prev = None;
        inorder(root, &mut prev, &mut min);
        min
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(2), Some(6), Some(1), Some(3)]);
    assert_eq!(Solution::min_diff_in_bst(root), 1);

    let root = TreeNode::from_vec(&[Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);
    assert_eq!(Solution::min_diff_in_bst(root), 1);
}
