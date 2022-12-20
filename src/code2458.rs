#![allow(dead_code)]

// 2458. Height of Binary Tree After Subtree Removal Queries
// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/
// https://leetcode.cn/problems/height-of-binary-tree-after-subtree-removal-queries/
//
// You are given the root of a binary tree with n nodes. Each node is assigned a unique value from 1 to n. You are also given an array queries of size m.
//
// You have to perform m independent queries on the tree where in the ith query you do the following:
//
// Remove the subtree rooted at the node with the value queries[i] from the tree. It is guaranteed that queries[i] will not be equal to the value of the root.
// Return an array answer of size m where answer[i] is the height of the tree after performing the ith query.
//
// Note:
//
// The queries are independent, so the tree returns to its initial state after each query.
// The height of a tree is the number of edges in the longest simple path from the root to some node in the tree.
//
// Example 1:
//
// Input: root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
// Output: [2]
// Explanation: The diagram above shows the tree after removing the subtree rooted at node with value 4.
// The height of the tree is 2 (The path 1 -> 3 -> 2).
//
// Example 2:
//
// Input: root = [5,8,9,2,1,3,7,4,6], queries = [3,2,4,8]
// Output: [3,2,3,2]
// Explanation: We have the following queries:
// - Removing the subtree rooted at node with value 3. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 4).
// - Removing the subtree rooted at node with value 2. The height of the tree becomes 2 (The path 5 -> 8 -> 1).
// - Removing the subtree rooted at node with value 4. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 6).
// - Removing the subtree rooted at node with value 8. The height of the tree becomes 2 (The path 5 -> 9 -> 3).
//
// Constraints:
//
// The number of nodes in the tree is n.
// 2 <= n <= 105
// 1 <= Node.val <= n
// All the values in the tree are unique.
// m == queries.length
// 1 <= m <= min(n, 104)
// 1 <= queries[i] <= n
// queries[i] != root.val
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            if root.is_none() {
                return 0;
            }
            let node = root.as_ref().unwrap().borrow();
            1 + count_nodes(&node.left) + count_nodes(&node.right)
        }

        fn dfs_left(root: &Option<Rc<RefCell<TreeNode>>>, h: usize, max_depth: &mut usize, height: &mut Vec<usize>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            height[node.val as usize] = height[node.val as usize].max(*max_depth);
            *max_depth = h.max(*max_depth);
            dfs_left(&node.left, h + 1, max_depth, height);
            dfs_left(&node.right, h + 1, max_depth, height);
        }

        fn dfs_right(root: &Option<Rc<RefCell<TreeNode>>>, h: usize, max_depth: &mut usize, height: &mut Vec<usize>) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            height[node.val as usize] = height[node.val as usize].max(*max_depth);
            *max_depth = h.max(*max_depth);
            dfs_right(&node.right, h + 1, max_depth, height);
            dfs_right(&node.left, h + 1, max_depth, height);
        }

        let n = count_nodes(&root);
        let (mut left, mut right) = (vec![0; n + 1], vec![0; n + 1]);
        let (mut a, mut b) = (0, 0);
        dfs_left(&root, 0, &mut a, &mut left);
        dfs_right(&root, 0, &mut b, &mut right);
        let mut ret = vec![];
        for q in queries {
            let q = q as usize;
            ret.push(left[q].max(right[q]) as i32);
        }
        ret
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(3),
        Some(4),
        Some(2),
        None,
        Some(6),
        Some(5),
        None,
        None,
        None,
        None,
        None,
        Some(7),
    ]);
    let queries = vec![4];
    let ret = Solution::tree_queries(root, queries);
    assert_eq!(ret, vec![2]);

    let root = TreeNode::from_vec(&[
        Some(5),
        Some(8),
        Some(9),
        Some(2),
        Some(1),
        Some(3),
        Some(7),
        Some(4),
        Some(6),
    ]);
    let queries = vec![3, 2, 4, 8];
    let ret = Solution::tree_queries(root, queries);
    assert_eq!(ret, vec![3, 2, 3, 2]);
}
