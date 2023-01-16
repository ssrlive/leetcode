#![allow(dead_code)]

// 2471. Minimum Number of Operations to Sort a Binary Tree by Level
// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
// https://leetcode.cn/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
//
// You are given the root of a binary tree with unique values.
//
// In one operation, you can choose any two nodes at the same level and swap their values.
//
// Return the minimum number of operations needed to make the values at each level sorted in a strictly increasing order.
//
// The level of a node is the number of edges along the path between it and the root node.
//
// Example 1:
//
// Input: root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
// Output: 3
// Explanation:
// - Swap 4 and 3. The 2nd level becomes [3,4].
// - Swap 7 and 5. The 3rd level becomes [5,6,8,7].
// - Swap 8 and 7. The 3rd level becomes [5,6,7,8].
// We used 3 operations so return 3.
// It can be proven that 3 is the minimum number of operations needed.
//
// Example 2:
//
// Input: root = [1,3,2,7,6,5,4]
// Output: 3
// Explanation:
// - Swap 3 and 2. The 2nd level becomes [2,3].
// - Swap 7 and 4. The 3rd level becomes [4,6,5,7].
// - Swap 6 and 5. The 3rd level becomes [4,5,6,7].
// We used 3 operations so return 3.
// It can be proven that 3 is the minimum number of operations needed.
//
// Example 3:
//
// Input: root = [1,2,3,4,5,6]
// Output: 0
// Explanation: Each level is already sorted in increasing order so return 0.
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 10^5].
// - 1 <= Node.val <= 10^5
// - All the values of the tree are unique.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn min_swaps(arr: &[i32]) -> i32 {
            let n = arr.len();
            let mut arr_pos = vec![(0, 0); n];
            for i in 0..n {
                arr_pos[i].0 = arr[i];
                arr_pos[i].1 = i;
            }
            arr_pos.sort();
            let mut vis = vec![false; n];
            let mut ans = 0;
            for i in 0..n {
                if vis[i] || arr_pos[i].1 == i {
                    continue;
                }
                let mut temp = 0;
                let mut j = i;
                while !vis[j] {
                    vis[j] = true;
                    j = arr_pos[j].1;
                    temp += 1;
                }
                if temp > 0 {
                    ans += temp - 1;
                }
            }
            ans
        }

        fn _minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            let mut q = VecDeque::new();
            q.push_back(root);
            let mut ans = 0;
            let mut v = Vec::new();
            while !q.is_empty() {
                v.clear();
                let sz = q.len();
                for _ in 0..sz {
                    let node = q.pop_front()?;
                    v.push(node.as_ref()?.borrow().val);
                    let left = node.as_ref()?.borrow().left.clone();
                    if left.is_some() {
                        q.push_back(left);
                    }
                    let right = node.as_ref()?.borrow().right.clone();
                    if right.is_some() {
                        q.push_back(right);
                    }
                }
                ans += min_swaps(&v);
            }
            Some(ans)
        }

        _minimum_operations(root).unwrap_or(0)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(4),
        Some(3),
        Some(7),
        Some(6),
        Some(8),
        Some(5),
        None,
        None,
        None,
        None,
        Some(9),
        None,
        Some(10),
    ]);
    assert_eq!(Solution::minimum_operations(root), 3);

    let root = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4)]);
    assert_eq!(Solution::minimum_operations(root), 3);

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
    assert_eq!(Solution::minimum_operations(root), 0);
}
