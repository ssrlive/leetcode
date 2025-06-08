#![allow(dead_code)]

// 3575. Maximum Good Subtree Score
// https://leetcode.com/problems/maximum-good-subtree-score/
// https://leetcode.cn/problems/maximum-good-subtree-score/
//
// Hard
//
// You are given an undirected tree rooted at node 0 with n nodes numbered from 0 to n - 1.
// Each node i has an integer value vals[i], and its parent is given by par[i].
//
// A subset of nodes within the subtree of a node is called good if every digit from 0 to 9 appears
// at most once in the decimal representation of the values of the selected nodes.
//
// The score of a good subset is the sum of the values of its nodes.
//
// Define an array maxScore of length n, where maxScore[u] represents the maximum possible sum of values of a good
// subset of nodes that belong to the subtree rooted at node u, including u itself and all its descendants.
//
// Return the sum of all values in maxScore.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: vals = [2,3], par = [-1,0]
//
// Output: 8
//
// Explanation:
//
// - The subtree rooted at node 0 includes nodes {0, 1}. The subset {2, 3} is good as the digits 2 and 3
//   appear only once. The score of this subset is 2 + 3 = 5.
// - The subtree rooted at node 1 includes only node {1}. The subset {3} is good. The score of this subset is 3.
// - The maxScore array is [5, 3], and the sum of all values in maxScore is 5 + 3 = 8. Thus, the answer is 8.
//
// Example 2:
//
// Input: vals = [1,5,2], par = [-1,0,0]
//
// Output: 15
//
// Explanation:
//
// - The subtree rooted at node 0 includes nodes {0, 1, 2}. The subset {1, 5, 2} is good as the digits 1, 5 and 2
//   appear only once. The score of this subset is 1 + 5 + 2 = 8.
// - The subtree rooted at node 1 includes only node {1}. The subset {5} is good. The score of this subset is 5.
// - The subtree rooted at node 2 includes only node {2}. The subset {2} is good. The score of this subset is 2.
// - The maxScore array is [8, 5, 2], and the sum of all values in maxScore is 8 + 5 + 2 = 15. Thus, the answer is 15.
//
// Example 3:
//
// Input: vals = [34,1,2], par = [-1,0,1]
//
// Output: 42
//
// Explanation:
//
// - The subtree rooted at node 0 includes nodes {0, 1, 2}. The subset {34, 1, 2} is good as the digits 3, 4, 1 and 2
//   appear only once. The score of this subset is 34 + 1 + 2 = 37.
// - The subtree rooted at node 1 includes node {1, 2}. The subset {1, 2} is good as the digits 1 and 2 appear only once.
//   The score of this subset is 1 + 2 = 3.
// - The subtree rooted at node 2 includes only node {2}. The subset {2} is good. The score of this subset is 2.
// - The maxScore array is [37, 3, 2], and the sum of all values in maxScore is 37 + 3 + 2 = 42. Thus, the answer is 42.
//
// Example 4:
//
// Input: vals = [3,22,5], par = [-1,0,1]
//
// Output: 18
//
// Explanation:
//
// - The subtree rooted at node 0 includes nodes {0, 1, 2}. The subset {3, 22, 5} is not good, as digit 2 appears twice.
//   Therefore, the subset {3, 5} is valid. The score of this subset is 3 + 5 = 8.
// - The subtree rooted at node 1 includes nodes {1, 2}. The subset {22, 5} is not good, as digit 2 appears twice.
//   Therefore, the subset {5} is valid. The score of this subset is 5.
// - The subtree rooted at node 2 includes {2}. The subset {5} is good. The score of this subset is 5.
// - The maxScore array is [8, 5, 5], and the sum of all values in maxScore is 8 + 5 + 5 = 18. Thus, the answer is 18.
//
// Constraints:
//
//     1 <= n == vals.length <= 500
//     1 <= vals[i] <= 10^9
//     par.length == n
//     par[0] == -1
//     0 <= par[i] < n for i in [1, n - 1]
//     The input is generated such that the parent array par represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn good_subtree_sum(vals: Vec<i32>, par: Vec<i32>) -> i32 {
        fn get_mask(mut num: i32) -> i32 {
            let mut nu = 0;
            while num > 0 {
                let digit = num % 10;
                if (1 << digit) & nu != 0 {
                    return -1; // Digit already exists
                }
                nu |= 1 << digit;
                num /= 10;
            }
            nu
        }

        let n = vals.len();
        let mod_val = 1_000_000_007;
        let mut ans = 0;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut sub_tree: Vec<Vec<usize>> = vec![vec![]; n];
        let mut mask: Vec<i32> = vec![0; n];
        let mut dp: Vec<i32> = vec![0; 1024];
        for i in 1..n {
            adj[i].push(par[i] as usize);
            adj[par[i] as usize].push(i);
        }
        for (i, &val) in vals.iter().enumerate() {
            mask[i] = get_mask(val);
        }

        #[allow(clippy::too_many_arguments)]
        fn dfs(
            node: usize,
            parent: i32,
            adj: &Vec<Vec<usize>>,
            vals: &Vec<i32>,
            mask: &Vec<i32>,
            sub_tree: &mut Vec<Vec<usize>>,
            dp: &mut Vec<i32>,
            ans: &mut i32,
            mod_val: i32,
        ) {
            sub_tree[node].push(node);
            dp.fill(0);

            for &neighbor in &adj[node] {
                if neighbor as i32 == parent {
                    continue;
                }
                dfs(neighbor, node as i32, adj, vals, mask, sub_tree, dp, ans, mod_val);
                for &itr in sub_tree[neighbor].clone().iter() {
                    sub_tree[node].push(itr);
                }
            }

            for &it in &sub_tree[node] {
                if mask[it] == -1 {
                    continue;
                }
                for i in 0..1024 {
                    if (i & mask[it]) == 0 {
                        dp[(i | mask[it]) as usize] = dp[(i | mask[it]) as usize].max((vals[it] + dp[i as usize]) % mod_val);
                    }
                }
            }

            *ans = (*ans + *dp.iter().max().unwrap()) % mod_val;
        }
        dfs(0, -1, &adj, &vals, &mask, &mut sub_tree, &mut dp, &mut ans, mod_val);
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::good_subtree_sum(vec![2, 3], vec![-1, 0]), 8);
    assert_eq!(Solution::good_subtree_sum(vec![1, 5, 2], vec![-1, 0, 0]), 15);
    assert_eq!(Solution::good_subtree_sum(vec![34, 1, 2], vec![-1, 0, 1]), 42);
    assert_eq!(Solution::good_subtree_sum(vec![3, 22, 5], vec![-1, 0, 1]), 18);
}
