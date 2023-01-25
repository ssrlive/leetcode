#![allow(dead_code)]

// 1315. Sum of Nodes with Even-Valued Grandparent
// https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/
// https://leetcode.cn/problems/sum-of-nodes-with-even-valued-grandparent/
//
// Medium
//
// Given the root of a binary tree, return the sum of values of nodes with an even-valued grandparent.
// If there are no nodes with an even-valued grandparent, return 0.
//
// A grandparent of a node is the parent of its parent if it exists.
//
// Example 1:
//
// Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
// Output: 18
// Explanation: The red nodes are the nodes with even-value grandparent while the blue nodes are the even-value grandparents.
//
// Example 2:
//
// Input: root = [1]
// Output: 0
//
// Constraints:
//
// -    The number of nodes in the tree is in the range [1, 10^4].
// -    1 <= Node.val <= 100
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn _sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            let mut sum = 0;
            let mut queue = std::collections::VecDeque::new();
            if root.is_some() {
                queue.push_back(root);
            }
            while let Some(node) = queue.pop_front() {
                if node.as_ref()?.borrow().val % 2 == 0 {
                    if let Some(left) = &node.as_ref()?.borrow().left {
                        if let Some(left_left) = &left.borrow().left {
                            sum += left_left.borrow().val;
                        }
                        if let Some(left_right) = &left.borrow().right {
                            sum += left_right.borrow().val;
                        }
                    }
                    if let Some(right) = &node.as_ref()?.borrow().right {
                        if let Some(right_left) = &right.borrow().left {
                            sum += right_left.borrow().val;
                        }
                        if let Some(right_right) = &right.borrow().right {
                            sum += right_right.borrow().val;
                        }
                    }
                }
                if let left @ Some(_) = &node.as_ref()?.borrow().left {
                    queue.push_back(left.clone());
                }
                if let right @ Some(_) = &node.as_ref()?.borrow().right {
                    queue.push_back(right.clone());
                }
            }
            Some(sum)
        }
        _sum_even_grandparent(root).unwrap_or(0)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(6),
        Some(7),
        Some(8),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(9),
        None,
        Some(1),
        Some(4),
        None,
        None,
        None,
        Some(5),
    ]);
    assert_eq!(Solution::sum_even_grandparent(root), 18);

    let root = TreeNode::from_vec(&[Some(1)]);
    assert_eq!(Solution::sum_even_grandparent(root), 0);
}
