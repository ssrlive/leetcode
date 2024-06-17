#![allow(dead_code)]

// 1448. Count Good Nodes in Binary Tree
// https://leetcode.com/problems/count-good-nodes-in-binary-tree/
// https://leetcode.cn/problems/count-good-nodes-in-binary-tree/
//
// Medium
//
// Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.
//
// Return the number of good nodes in the binary tree.
//
// Example 1:
//
// Input: root = [3,1,4,3,null,1,5]
// Output: 4
// Explanation: Nodes in blue are good.
// Root Node (3) is always a good node.
// Node 4 -> (3,4) is the maximum value in the path starting from the root.
// Node 5 -> (3,4,5) is the maximum value in the path
// Node 3 -> (3,1,3) is the maximum value in the path.
//
// Example 2:
//
// Input: root = [3,3,null,4,2]
// Output: 3
// Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.
//
// Example 3:
//
// Input: root = [1]
// Output: 1
// Explanation: Root is considered as good.
//
// Constraints:
//
// -    The number of nodes in the binary tree is in the range [1, 10^5].
// -    Each node's value is between [-10^4, 10^4].
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();
                let val = node.val;
                let mut count = 0;
                if val >= max {
                    count += 1;
                }
                count += dfs(&node.left, std::cmp::max(max, val));
                count += dfs(&node.right, std::cmp::max(max, val));
                count
            } else {
                0
            }
        }
        dfs(&root, i32::MIN)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_string("[3,1,4,3,null,1,5]");
    assert_eq!(Solution::good_nodes(root), 4);

    let root = TreeNode::from_string("[3,3,null,4,2]");
    assert_eq!(Solution::good_nodes(root), 3);

    let root = TreeNode::from_string("[1]");
    assert_eq!(Solution::good_nodes(root), 1);
}
