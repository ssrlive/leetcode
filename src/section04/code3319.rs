#![allow(dead_code)]

// 3319. K-th Largest Perfect Subtree Size in Binary Tree
// https://leetcode.com/problems/k-th-largest-perfect-subtree-size-in-binary-tree/
// https://leetcode.cn/problems/k-th-largest-perfect-subtree-size-in-binary-tree/
//
// Medium
//
// You are given the root of a binary tree and an integer k.
//
// Return an integer denoting the size of the kth largest perfect binary subtree, or -1 if it doesn't exist.
//
// A perfect binary tree is a tree where all leaves are on the same level, and every parent has two children.
//
// Example 1:
//
// Input: root = [5,3,6,5,2,5,7,1,8,null,null,6,8], k = 2
//
// Output: 3
//
// Explanation:
//
// The roots of the perfect binary subtrees are highlighted in black. Their sizes, in decreasing order are [3, 3, 1, 1, 1, 1, 1, 1].
// The 2nd largest size is 3.
//
// Example 2:
//
// Input: root = [1,2,3,4,5,6,7], k = 1
//
// Output: 7
//
// Explanation:
//
// The sizes of the perfect binary subtrees in decreasing order are [7, 3, 3, 1, 1, 1, 1]. The size of the largest perfect binary subtree is 7.
//
// Example 3:
//
// Input: root = [1,2,3,null,4], k = 3
//
// Output: -1
//
// Explanation:
//
// The sizes of the perfect binary subtrees in decreasing order are [1, 1]. There are fewer than 3 perfect binary subtrees.
//
// Constraints:
//
//     The number of nodes in the tree is in the range [1, 2000].
//     1 <= Node.val <= 2000
//     1 <= k <= 1024
//

struct Solution;

use crate::treenode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn make_tree(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) -> (bool, i32) {
            if let Some(node) = root {
                let node = node.borrow();
                let (l, r) = (make_tree(node.left.clone(), ans), make_tree(node.right.clone(), ans));
                if l.0 && r.0 && l.1 == r.1 {
                    let s = l.1 + r.1 + 1;
                    ans.push(s);
                    (true, s)
                } else {
                    (false, 0)
                }
            } else {
                (true, 0)
            }
        }

        let mut ans = vec![];
        make_tree(root, &mut ans);

        ans.sort_unstable_by(|a, b| b.cmp(a));
        ans.get((k - 1) as usize).copied().unwrap_or(-1)
    }
}

#[test]
fn test() {
    use crate::treenode::TreeNode;
    let root = TreeNode::from_vec(&[
        Some(5),
        Some(3),
        Some(6),
        Some(5),
        Some(2),
        Some(5),
        Some(7),
        Some(1),
        Some(8),
        None,
        None,
        Some(6),
        Some(8),
    ]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 2), 3);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 1), 7);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::kth_largest_perfect_subtree(root, 3), -1);
}
