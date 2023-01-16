#![allow(dead_code)]

// 897. Increasing Order Search Tree
// https://leetcode.com/problems/increasing-order-search-tree/
// https://leetcode.cn/problems/increasing-order-search-tree/
//
// Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree
// is now the root of the tree, and every node has no left child and only one right child.
//
// Example 1:
//
// Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
// Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
//
// Example 2:
//
// Input: root = [5,1,7]
// Output: [1,null,5,null,7]
//
// Constraints:
//
// - The number of nodes in the given tree will be in the range [1, 100].
// - 0 <= Node.val <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(
            root: Option<Rc<RefCell<TreeNode>>>,
            tmp: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root.clone() {
                let mut node = node.borrow_mut();
                let res = inorder(node.left.take(), root);
                node.right = inorder(node.right.take(), tmp);
                res
            } else {
                tmp
            }
        }
        inorder(root, None)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(5),
        Some(3),
        Some(6),
        Some(2),
        Some(4),
        None,
        Some(8),
        Some(1),
        None,
        None,
        None,
        Some(7),
        Some(9),
    ]);
    let res = Solution::increasing_bst(root);
    let expected = vec![
        Some(1),
        None,
        Some(2),
        None,
        Some(3),
        None,
        Some(4),
        None,
        Some(5),
        None,
        Some(6),
        None,
        Some(7),
        None,
        Some(8),
        None,
        Some(9),
    ];
    assert_eq!(res.as_ref().unwrap().borrow().to_vec(), expected);

    let root = TreeNode::from_vec(&[Some(5), Some(1), Some(7)]);
    let res = Solution::increasing_bst(root);
    let expected = vec![Some(1), None, Some(5), None, Some(7)];
    assert_eq!(res.as_ref().unwrap().borrow().to_vec(), expected);
}
