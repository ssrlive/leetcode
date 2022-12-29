#![allow(dead_code)]

// 865. Smallest Subtree with all the Deepest Nodes
// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
// https://leetcode.cn/problems/smallest-subtree-with-all-the-deepest-nodes/
//
// Given the root of a binary tree, the depth of each node is the shortest distance to the root.
//
// Return the smallest subtree such that it contains all the deepest nodes in the original tree.
//
// A node is called the deepest if it has the largest depth possible among any node in the entire tree.
//
// The subtree of a node is a tree consisting of that node, plus the set of all descendants of that node.
//
// Example 1:
//
// Input: root = [3,5,1,6,2,0,8,null,null,7,4]
// Output: [2,7,4]
// Explanation: We return the node with value 2, colored in yellow in the diagram.
// The nodes coloured in blue are the deepest nodes of the tree.
// Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them, so we return it.
//
// Example 2:
//
// Input: root = [1]
// Output: [1]
// Explanation: The root is the deepest node in the tree.
//
// Example 3:
//
// Input: root = [0,1,3,null,2]
// Output: [2]
// Explanation: The deepest node in the tree is 2, the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
//
// Constraints:
//
// - The number of nodes in the tree will be in the range [1, 500].
// - 0 <= Node.val <= 500
// - The values of the nodes in the tree are unique.
//
// Note: This question is the same as 1123: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
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
    let result = Solution::subtree_with_all_deepest(root);
    assert_eq!(result.unwrap().borrow().val, 2);

    let root = TreeNode::from_vec(&[Some(1)]);
    let result = Solution::subtree_with_all_deepest(root);
    assert_eq!(result.unwrap().borrow().val, 1);

    let root = TreeNode::from_vec(&[Some(0), Some(1), Some(3), None, Some(2)]);
    let result = Solution::subtree_with_all_deepest(root);
    assert_eq!(result.unwrap().borrow().val, 2);
}
