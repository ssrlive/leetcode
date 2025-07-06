#![allow(dead_code)]

// 863. All Nodes Distance K in Binary Tree
// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
// https://leetcode.cn/problems/all-nodes-distance-k-in-binary-tree/
//
// Given the root of a binary tree, the value of a target node target, and an integer k, return an array of the values of all nodes that have a distance k from the target node.
//
// You can return the answer in any order.
//
// Example 1:
//
// Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
// Output: [7,4,1]
// Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
//
// Example 2:
//
// Input: root = [1], target = 1, k = 3
// Output: []
//
// Constraints:
//
// - The number of nodes in the tree is in the range [1, 500].
// - 0 <= Node.val <= 500
// - All the values Node.val are unique.
// - target is the value of one of the nodes in the tree.
// - 0 <= k <= 1000
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        use std::collections::{HashMap, HashSet, VecDeque};

        fn dfs(node: Rc<RefCell<TreeNode>>, chd_to_prnt: &mut HashMap<i32, Rc<RefCell<TreeNode>>>) {
            if let Some(left) = node.borrow().left.clone() {
                chd_to_prnt.insert(left.borrow().val, node.clone());
                dfs(left, chd_to_prnt);
            }
            if let Some(right) = node.borrow().right.clone() {
                chd_to_prnt.insert(right.borrow().val, node.clone());
                dfs(right, chd_to_prnt);
            }
        }

        let mut ans: Vec<i32> = Vec::new();
        if let Some(node) = root {
            let chd_to_prnt: HashMap<i32, Rc<RefCell<TreeNode>>> = {
                let mut ch_to_prnt: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
                dfs(node, &mut ch_to_prnt);
                ch_to_prnt
            };
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            let mut seen: HashSet<i32> = HashSet::new();
            if let Some(t) = target {
                queue.push_back(t.clone());
                let value = t.borrow().val;
                seen.insert(value);
                // sanity check, required
                if k == 0 {
                    return vec![value];
                }
            }
            let mut dist: u16 = 0;
            while !queue.is_empty() {
                let len_q: usize = queue.len();
                for _ in 0..len_q {
                    if let Some(cur) = queue.pop_front() {
                        ans.push(cur.borrow().val);
                        if let Some(left) = cur.clone().borrow().left.clone()
                            && seen.insert(left.clone().borrow().val)
                        {
                            queue.push_back(left);
                        }
                        if let Some(right) = cur.clone().borrow().right.clone()
                            && seen.insert(right.clone().borrow().val)
                        {
                            queue.push_back(right);
                        }
                        if let Some(prnt) = chd_to_prnt.get(&cur.clone().borrow().val)
                            && seen.insert(prnt.clone().borrow().val)
                        {
                            queue.push_back(prnt.clone());
                        }
                    }
                }
                if dist == k as u16 {
                    break;
                }
                dist += 1;
                ans.clear();
            }
        }
        ans
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
    let target = TreeNode::find_node(&root, 5);
    let k = 2;
    let ans = vec![7, 4, 1];
    assert_eq!(Solution::distance_k(root, target, k), ans);

    let root = TreeNode::from_vec(&[Some(1)]);
    let target = TreeNode::find_node(&root, 1);
    let k = 3;
    let ans: Vec<i32> = Vec::new();
    assert_eq!(Solution::distance_k(root, target, k), ans);
}
