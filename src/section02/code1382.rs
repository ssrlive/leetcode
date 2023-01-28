#![allow(dead_code)]

// 1382. Balance a Binary Search Tree
// https://leetcode.com/problems/balance-a-binary-search-tree/
// https://leetcode.cn/problems/balance-a-binary-search-tree/
//
// Medium
//
// Given the root of a binary search tree, return a balanced binary search tree with the same node values.
// If there is more than one answer, return any of them.
//
// A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.
//
// Example 1:
//
// Input: root = [1,null,2,null,3,null,4,null,null]
// Output: [2,1,3,null,null,null,4]
// Explanation: This is not the only correct answer, [3,1,4,null,2] is also correct.
//
// Example 2:
//
// Input: root = [2,1,3]
// Output: [2,1,3]
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 10^4].
// -    1 <= Node.val <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();
                inorder(&node.left, v);
                v.push(node.val);
                inorder(&node.right, v);
            }
        }

        fn build(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if v.is_empty() {
                return None;
            }
            let mid = v.len() / 2;
            let mut node = TreeNode::new(v[mid]);
            node.left = build(&v[..mid]);
            node.right = build(&v[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }

        let mut v = vec![];
        inorder(&root, &mut v);
        build(&v)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[1,null,2,null,3,null,4,null,null]");
    let root = Solution::balance_bst(root);
    let root = TreeNode::to_string(&root);
    assert_eq!(root, "[3, 2, 4, 1]");

    let root = TreeNode::from_string("[2,1,3]");
    let root = Solution::balance_bst(root);
    let root = TreeNode::to_string(&root);
    assert_eq!(root, "[2, 1, 3]");
}
