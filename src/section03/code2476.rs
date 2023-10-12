#![allow(dead_code)]

// 2476. Closest Nodes Queries in a Binary Search Tree
// https://leetcode.com/problems/closest-nodes-queries-in-a-binary-search-tree/
// https://leetcode.cn/problems/closest-nodes-queries-in-a-binary-search-tree/
//
// You are given the root of a binary search tree and an array queries of size n consisting of positive integers.
//
// Find a 2D array answer of size n where answer[i] = [mini, maxi]:
//
// - mini is the largest value in the tree that is smaller than or equal to queries[i]. If a such value does not exist, add -1 instead.
// - maxi is the smallest value in the tree that is greater than or equal to queries[i]. If a such value does not exist, add -1 instead.
//
// Return the array answer.
//
// Example 1:
//
// Input: root = [6,2,13,1,4,9,15,null,null,null,null,null,null,14], queries = [2,5,16]
// Output: [[2,2],[4,6],[15,-1]]
// Explanation: We answer the queries in the following way:
// - The largest number that is smaller or equal than 2 in the tree is 2, and the smallest number that is greater or equal than 2 is still 2. So the answer for the first query is [2,2].
// - The largest number that is smaller or equal than 5 in the tree is 4, and the smallest number that is greater or equal than 5 is 6. So the answer for the second query is [4,6].
// - The largest number that is smaller or equal than 16 in the tree is 15, and the smallest number that is greater or equal than 16 does not exist. So the answer for the third query is [15,-1].
//
// Example 2:
//
// Input: root = [4,null,9], queries = [3]
// Output: [[-1,4]]
// Explanation: The largest number that is smaller or equal to 3 in the tree does not exist, and the smallest number that is greater or equal to 3 is 4. So the answer for the query is [-1,4].
//
// Constraints:
//
// - The number of nodes in the tree is in the range [2, 10^5].
// - 1 <= Node.val <= 10^6
// - n == queries.length
// - 1 <= n <= 10^5
// - 1 <= queries[i] <= 10^6
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(node: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let node = n.borrow();
                helper(node.left.as_ref(), res);
                res.push(node.val);
                helper(node.right.as_ref(), res);
            }
        }

        let mut sorted = vec![];
        helper(root.as_ref(), &mut sorted);
        let mut res = vec![];
        for query in queries {
            let r = sorted.binary_search(&query);
            match r {
                Ok(i) => {
                    res.push(vec![sorted[i], sorted[i]]);
                }
                Err(i) => {
                    if i == 0 {
                        res.push(vec![-1, sorted[i]]);
                    } else if i == sorted.len() {
                        res.push(vec![sorted[i - 1], -1]);
                    } else {
                        res.push(vec![sorted[i - 1], sorted[i]]);
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(6),
        Some(2),
        Some(13),
        Some(1),
        Some(4),
        Some(9),
        Some(15),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(14),
        None,
    ]);
    let queries = vec![2, 5, 16];
    let result = Solution::closest_nodes(root, queries);
    assert_eq!(result, vec![vec![2, 2], vec![4, 6], vec![15, -1]]);

    let root = TreeNode::from_vec(&[Some(4), None, Some(9)]);
    let queries = vec![3];
    let result = Solution::closest_nodes(root, queries);
    assert_eq!(result, vec![vec![-1, 4]]);
}
