#![allow(dead_code)]

// 1161. Maximum Level Sum of a Binary Tree
// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
// https://leetcode.cn/problems/maximum-level-sum-of-a-binary-tree/
//
// Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
//
// Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
//
// Example 1:
//
// Input: root = [1,7,0,7,-8,null,null]
// Output: 2
// Explanation:
// Level 1 sum = 1.
// Level 2 sum = 7 + 0 = 7.
// Level 3 sum = 7 + -8 = -1.
// So we return the level with the maximum sum which is level 2.
//
// Example 2:
//
// Input: root = [989,null,10250,98693,-89388,null,null,null,-32127]
// Output: 2
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^4].
// - -10^5 <= Node.val <= 10^5
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let root = match root {
            Some(a) => a,
            None => return 0,
        };

        let mut level = 1;
        let mut res = 0;
        let mut max = std::i32::MIN;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let mut sum = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                sum += node.borrow().val;
                if let Some(ref l) = node.borrow().left {
                    queue.push_back(l.clone());
                };
                if let Some(ref r) = node.borrow().right {
                    queue.push_back(r.clone());
                };
            }
            if sum > max {
                max = sum;
                res = level;
            }
            level += 1;
        }
        res
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[Some(1), Some(7), Some(0), Some(7), Some(-8), None, None]);
    assert_eq!(Solution::max_level_sum(root), 2);

    let root = TreeNode::from_vec(&[
        Some(989),
        None,
        Some(10250),
        Some(98693),
        Some(-89388),
        None,
        None,
        None,
        Some(-32127),
    ]);
    assert_eq!(Solution::max_level_sum(root), 2);
}
