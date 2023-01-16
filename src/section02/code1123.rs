#![allow(dead_code)]

// 1123. Lowest Common Ancestor of Deepest Leaves
// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
// https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/
//
// Given the root of a binary tree, return the lowest common ancestor of its deepest leaves.
//
// Recall that:
//
// The node of a binary tree is a leaf if and only if it has no children
// The depth of the root of the tree is 0. if the depth of a node is d, the depth of each of its children is d + 1.
// The lowest common ancestor of a set S of nodes, is the node A with the largest depth such that every node in S is in the subtree with root A.
//
// Example 1:
//
// Input: root = [3,5,1,6,2,0,8,null,null,7,4]
// Output: [2,7,4]
// Explanation: We return the node with value 2, colored in yellow in the diagram.
// The nodes coloured in blue are the deepest leaf-nodes of the tree.
// Note that nodes 6, 0, and 8 are also leaf nodes, but the depth of them is 2, but the depth of nodes 7 and 4 is 3.
//
// Example 2:
//
// Input: root = [1]
// Output: [1]
// Explanation: The root is the deepest node in the tree, and it's the lca of itself.
//
// Example 3:
//
// Input: root = [0,1,3,null,2]
// Output: [2]
// Explanation: The deepest leaf node in the tree is 2, the lca of one node is itself.
//
// Constraints:
//
// - The number of nodes in the tree will be in the range [1, 1000].
// - 0 <= Node.val <= 1000
// - The values of the nodes in the tree are unique.
//
// Note: This question is the same as 865: https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Option<(Option<Rc<RefCell<TreeNode>>>, i32)> {
            if node.is_none() {
                return Some((node, -1));
            }
            if node.as_ref()?.borrow().left.is_none() && node.as_ref()?.borrow().right.is_none() {
                return Some((node, 0));
            }

            let (right, depth_right_subtree) = dfs(node.as_ref()?.borrow().right.clone())?;
            let (left, depth_left_subtree) = dfs(node.as_ref()?.borrow().left.clone())?;

            let depth_right_subtree = depth_right_subtree + 1;
            let depth_left_subtree = depth_left_subtree + 1;

            match depth_left_subtree.cmp(&depth_right_subtree) {
                std::cmp::Ordering::Equal => Some((node, depth_right_subtree)),
                std::cmp::Ordering::Greater => Some((left, depth_left_subtree)),
                std::cmp::Ordering::Less => Some((right, depth_right_subtree)),
            }
        }

        if root.is_none() {
            return root;
        }
        if root.as_ref()?.borrow().left.is_none() && root.as_ref()?.borrow().right.is_none() {
            return root;
        }

        let (right, depth_right_subtree) = dfs(root.as_ref()?.borrow().right.clone())?;
        let (left, depth_left_subtree) = dfs(root.as_ref()?.borrow().left.clone())?;

        let depth_right_subtree = depth_right_subtree + 1;
        let depth_left_subtree = depth_left_subtree + 1;

        match depth_left_subtree.cmp(&depth_right_subtree) {
            std::cmp::Ordering::Equal => root,
            std::cmp::Ordering::Greater => left,
            std::cmp::Ordering::Less => right,
        }
    }
}

#[test]
fn test() {
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
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(result, TreeNode::from_vec(&[Some(2), Some(7), Some(4)]));

    let root = TreeNode::from_vec(&[Some(1)]);
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(result, TreeNode::from_vec(&[Some(1)]));

    let root = TreeNode::from_vec(&[Some(0), Some(1), Some(3), None, Some(2)]);
    let result = Solution::lca_deepest_leaves(root);
    assert_eq!(result, TreeNode::from_vec(&[Some(2)]));
}
