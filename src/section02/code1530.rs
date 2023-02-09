#![allow(dead_code)]

/*

// 1530. Number of Good Leaf Nodes Pairs
// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
// https://leetcode.cn/problems/number-of-good-leaf-nodes-pairs/
//
// Medium
//
// You are given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a binary tree is said to be good if the length of the shortest path between them is less than or equal to distance.

Return the number of good leaf node pairs in the tree.

Example 1:

Input: root = [1,2,3,null,4], distance = 3
Output: 1
Explanation: The leaf nodes of the tree are 3 and 4 and the length of the shortest path between them is 3. This is the only good pair.

Example 2:

Input: root = [1,2,3,4,5,6,7], distance = 3
Output: 2
Explanation: The good pairs are [4,5] and [6,7] with shortest path = 2. The pair [4,6] is not good because the length of ther shortest path between them is 4.

Example 3:

Input: root = [7,1,4,6,null,5,3,null,null,null,null,null,2], distance = 3
Output: 1
Explanation: The only good pair is [2,5].

Constraints:

    The number of nodes in the tree is in the range [1, 2^10].
    1 <= Node.val <= 100
    1 <= distance <= 10
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut result = 0;
        Self::dfs(&root, distance, &mut result);
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, distance: i32, result: &mut i32) -> Vec<i32> {
        if node.is_some() {
            let node = node.as_ref().unwrap().borrow();
            let left = Self::dfs(&node.left, distance, result);
            let right = Self::dfs(&node.right, distance, result);
            let mut leafs = vec![0; distance as usize + 1];
            for (i, &item_l) in left.iter().enumerate() {
                for (j, &item_r) in right.iter().enumerate() {
                    if i + j + 2 <= distance as usize {
                        *result += item_l * item_r;
                    }
                }
            }
            for i in 0..left.len() {
                if i + 1 < leafs.len() {
                    leafs[i + 1] += left[i];
                }
            }
            for i in 0..right.len() {
                if i + 1 < leafs.len() {
                    leafs[i + 1] += right[i];
                }
            }
            if node.left.is_none() && node.right.is_none() {
                leafs[0] = 1;
            }
            leafs
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (TreeNode::from_string("[1,2,3,null,4]"), 3, 1),
        (TreeNode::from_string("[1,2,3,4,5,6,7]"), 3, 2),
        (
            TreeNode::from_string("[7,1,4,6,null,5,3,null,null,null,null,null,2]"),
            3,
            1,
        ),
    ];
    for (root, distance, expected) in cases {
        assert_eq!(Solution::count_pairs(root, distance), expected);
    }
}
