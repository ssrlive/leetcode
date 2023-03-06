#![allow(dead_code)]

/*

// 2265. Count Nodes Equal to Average of Subtree
// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
// https://leetcode.cn/problems/count-nodes-equal-to-average-of-subtree/
//
// Medium
//
// Given the root of a binary tree, return the number of nodes where the value of the node is equal to the average of the values in its subtree.

Note:

    The average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
    A subtree of root is a tree consisting of root and all of its descendants.

Example 1:

Input: root = [4,8,5,0,1,null,6]
Output: 5
Explanation:
For the node with value 4: The average of its subtree is (4 + 8 + 5 + 0 + 1 + 6) / 6 = 24 / 6 = 4.
For the node with value 5: The average of its subtree is (5 + 6) / 2 = 11 / 2 = 5.
For the node with value 0: The average of its subtree is 0 / 1 = 0.
For the node with value 1: The average of its subtree is 1 / 1 = 1.
For the node with value 6: The average of its subtree is 6 / 1 = 6.

Example 2:

Input: root = [1]
Output: 1
Explanation: For the node with value 1: The average of its subtree is 1 / 1 = 1.

Constraints:

    The number of nodes in the tree is in the range [1, 1000].
    0 <= Node.val <= 1000
*/

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn avg(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> (i32, i32) {
            if node.is_none() {
                return (0, 0);
            }
            let node = node.as_ref().unwrap().borrow();
            let mut sum = node.val;
            let mut nodes = 1;
            if let ref left @ Some(_) = node.left {
                let (sub_sum, sub_nodes) = avg(left, count);
                sum += sub_sum;
                nodes += sub_nodes;
            }
            if let ref right @ Some(_) = node.right {
                let (sub_sum, sub_nodes) = avg(right, count);
                sum += sub_sum;
                nodes += sub_nodes;
            }
            if sum / nodes == node.val {
                *count += 1;
            }
            (sum, nodes)
        }

        pub fn count_nodes_equal_to_avg(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut count = 0;
            let _ = avg(node, &mut count);
            count
        }

        count_nodes_equal_to_avg(&root)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)]);
    assert_eq!(Solution::average_of_subtree(root), 5);

    let root = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::average_of_subtree(root), 1);
}
