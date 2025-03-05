#![allow(dead_code)]

// 236. Lowest Common Ancestor of a Binary Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/
//
// Medium
//
// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined
// between two nodes p and q as the lowest node in T that has both p and q as descendants
// (where we allow a node to be a descendant of itself).”
//
// Example 1:
// Given the following binary tree:  root = [3,5,1,6,2,0,8,null,null,7,4]
//
//         3
//        / \
//       5   1
//      / \ / \
//     6  2 0  8
//       / \
//      7   4
//
// Input: root, p = 5, q = 1
// Output: 3
// Explanation: The LCA of nodes 5 and 1 is 3.
//
// Example 2:
// Given the following binary tree:  root = [3,5,1,6,2,0,8,null,null,7,4]
//
//         3
//        / \
//       5   1
//      / \ / \
//     6  2 0  8
//       / \
//      7   4
//
// Input: root, p = 5, q = 4
// Output: 5
// Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself
// according to the LCA definition.
//
// Note:
// All of the nodes' values will be unique.
// p and q are different and both values will exist in the binary tree.
//

use crate::treenode::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        fn post_order_traverse(node: OptNode, p: OptNode, q: OptNode) -> OptNode {
            node.as_ref()?;
            if node == p || node == q {
                return node;
            }
            let left = post_order_traverse(node.as_ref()?.borrow().left.clone(), p.clone(), q.clone());
            let right = post_order_traverse(node.as_ref()?.borrow().right.clone(), p, q);
            match (left.is_some(), right.is_some()) {
                (true, true) => node,
                (false, false) => None,
                (true, false) => left,
                (false, true) => right,
            }
        }
        post_order_traverse(root, p, q)
    }

    pub fn lowest_common_ancestor_2(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        type OptNode = Option<Rc<RefCell<TreeNode>>>;
        pub fn _lowest_common_ancestor(root: OptNode, p: OptNode, q: OptNode) -> OptNode {
            root.as_ref()?;
            if root == p || root == q {
                return root;
            }
            let left0 = root.as_ref()?.borrow().left.clone();
            let left = _lowest_common_ancestor(left0, p.clone(), q.clone());
            if left.is_some() {
                let right0 = root.as_ref()?.borrow().right.clone();
                let right = _lowest_common_ancestor(right0, p, q);
                if right.is_some() { root } else { left }
            } else {
                let right = root.as_ref()?.borrow().right.clone();
                _lowest_common_ancestor(right, p, q)
            }
        }

        _lowest_common_ancestor(root, p, q)
    }
}

#[test]
fn test_lowest_common_ancestor() {
    fn test() -> Option<()> {
        let root = TreeNode::from_vec(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);

        let node5 = TreeNode::find_node(&root, 5);
        let node1 = TreeNode::find_node(&root, 1);
        let node4 = TreeNode::find_node(&root, 4);

        let node = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node1.clone());
        let val = node.as_ref()?.borrow().val;
        assert_eq!(val, 3);

        let node = Solution::lowest_common_ancestor(root.clone(), node5.clone(), node4.clone());
        let val = node.as_ref()?.borrow().val;
        assert_eq!(val, 5);
        Some(())
    }
    test().unwrap();
}
