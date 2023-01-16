#![allow(dead_code)]

// 222. Count Complete Tree Nodes
// https://leetcode.com/problems/count-complete-tree-nodes/
// https://leetcode.cn/problems/count-complete-tree-nodes/
//
// Given a complete binary tree, count the number of nodes.
//
// Note:
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn count_recurse(node: &Option<Rc<RefCell<TreeNode>>>, c: &mut i32) {
            if let Some(node) = node {
                *c += 1;
                let node = node.borrow();
                count_recurse(&node.left, c);
                count_recurse(&node.right, c);
            }
        }
        let res = &mut 0_i32;
        count_recurse(&root, res);
        *res
    }
}

#[test]
fn test_count_nodes() {
    let tree = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::count_nodes(tree), 6);

    let tree = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::count_nodes(tree), 5);

    let tree = TreeNode::from_vec(&[Some(11), Some(12), Some(13), Some(14), Some(15), Some(16)]);
    assert_eq!(Solution::count_nodes(tree), 6);

    let tree = TreeNode::from_vec(&[Some(1), Some(2), Some(16), Some(17)]);
    assert_eq!(Solution::count_nodes(tree), 4);
}
