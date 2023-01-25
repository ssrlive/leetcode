#![allow(dead_code)]

// 1325. Delete Leaves With a Given Value
// https://leetcode.com/problems/delete-leaves-with-a-given-value/
// https://leetcode.cn/problems/delete-leaves-with-a-given-value/
//
// Medium
//
// Given a binary tree root and an integer target, delete all the leaf nodes with value target.
//
// Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).
//
// Example 1:
//
// Input: root = [1,2,3,2,null,2,4], target = 2
// Output: [1,null,3,null,4]
// Explanation: Leaf nodes in green with value (target = 2) are removed (Picture in left).
// After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).
//
// Example 2:
//
// Input: root = [1,3,3,3,2], target = 3
// Output: [1,3,null,null,2]
//
// Example 3:
//
// Input: root = [1,2,null,2,null,2], target = 2
// Output: [1]
// Explanation: Leaf nodes in green with value (target = 2) are removed at each step.
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 3000].
// -    1 <= Node.val, target <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        fn _remove_leaf_nodes(root: OptNode, target: i32) -> OptNode {
            if root.is_some() {
                let left = _remove_leaf_nodes(root.as_ref()?.borrow_mut().left.take(), target);
                let right = _remove_leaf_nodes(root.as_ref()?.borrow_mut().right.take(), target);
                if root.as_ref()?.borrow().val == target && left.is_none() && right.is_none() {
                    return None;
                }
                root.as_ref()?.borrow_mut().right = right;
                root.as_ref()?.borrow_mut().left = left;
                return root;
            }
            None
        }
        _remove_leaf_nodes(root, target)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)],
            2,
            vec![Some(1), None, Some(3), None, Some(4)],
        ),
        (
            vec![Some(1), Some(3), Some(3), Some(3), Some(2)],
            3,
            vec![Some(1), Some(3), None, None, Some(2)],
        ),
        (vec![Some(1), Some(2), None, Some(2), None, Some(2)], 2, vec![Some(1)]),
    ];
    for (root, target, expect) in cases {
        let root = TreeNode::from_vec(&root);
        let expect = TreeNode::from_vec(&expect);
        let output = Solution::remove_leaf_nodes(root, target);
        assert_eq!(output, expect);
    }
}
