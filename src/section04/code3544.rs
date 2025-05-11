#![allow(dead_code)]

// 3544. Subtree Inversion Sum
// https://leetcode.com/problems/subtree-inversion-sum
// https://leetcode.cn/problems/subtree-inversion-sum
//
// Hard
//
// You are given an undirected tree rooted at node 0, with n nodes numbered from 0 to n - 1.
// The tree is represented by a 2D integer array edges of length n - 1,
// where edges[i] = [ui, vi] indicates an edge between nodes ui and vi.
//
// You are also given an integer array nums of length n, where nums[i] represents the value at node i, and an integer k.
//
// You may perform inversion operations on a subset of nodes subject to the following rules:
//
//     Subtree Inversion Operation:
//
//         When you invert a node, every value in the
//
//         rooted at that node is multiplied by -1.
//
//     Distance Constraint on Inversions:
//
//         You may only invert a node if it is "sufficiently far" from any other inverted node.
//
//         Specifically, if you invert two nodes a and b such that one is an ancestor of the other (i.e., if LCA(a, b) = a or LCA(a, b) = b),
//         then the distance (the number of edges on the unique path between them) must be at least k.
//
// Return the maximum possible sum of the tree's node values after applying inversion operations.
//
// Example 1:
//
// Input: edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]], nums = [4,-8,-6,3,7,-2,5], k = 2
//
// Output: 27
//
// Explanation:
//
//     Apply inversion operations at nodes 0, 3, 4 and 6.
//     The final nums array is [-4, 8, 6, 3, 7, 2, 5], and the total sum is 27.
//
// Example 2:
//
// Input: edges = [[0,1],[1,2],[2,3],[3,4]], nums = [-1,3,-2,4,-5], k = 2
//
// Output: 9
//
// Explanation:
//
//     Apply the inversion operation at node 4.
//     The final nums array becomes [-1, 3, -2, 4, 5], and the total sum is 9.
//
// Example 3:
//
// Input: edges = [[0,1],[0,2]], nums = [0,-1,-2], k = 3
//
// Output: 3
//
// Explanation:
//
// Apply inversion operations at nodes 1 and 2.
//
// Constraints:
//
//     2 <= n <= 5 * 10^4
//     edges.length == n - 1
//     edges[i] = [ui, vi]
//     0 <= ui, vi < n
//     nums.length == n
//     -5 * 10^4 <= nums[i] <= 5 * 10^4
//     1 <= k <= 50
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        fn dfs(
            node: i64,
            dist_since_inversion: i64,
            parent: i64,
            adj: &[Vec<i64>],
            nums: &[i64],
            k: i64,
            memo: &mut Vec<Vec<(i64, i64)>>,
        ) -> (i64, i64) {
            if memo[node as usize][dist_since_inversion as usize].0 != i64::MIN {
                return memo[node as usize][dist_since_inversion as usize];
            }

            // Do not invert the current node
            let mut min_sum = nums[node as usize];
            let mut max_sum = nums[node as usize];
            for &neighbor in &adj[node as usize] {
                if neighbor != parent {
                    let (child_min, child_max) = dfs(neighbor, (dist_since_inversion + 1).min(k), node, adj, nums, k, memo);
                    min_sum += child_min;
                    max_sum += child_max;
                }
            }

            // Invert the current node (if distance constraint allows)
            if dist_since_inversion >= k {
                let mut min_sum_inv = -nums[node as usize];
                let mut max_sum_inv = -nums[node as usize];
                for &neighbor in &adj[node as usize] {
                    if neighbor != parent {
                        let (child_min, child_max) = dfs(neighbor, 1, node, adj, nums, k, memo);
                        min_sum_inv += -child_max; // Invert child's min/max
                        max_sum_inv += -child_min;
                    }
                }
                min_sum = min_sum.min(min_sum_inv);
                max_sum = max_sum.max(max_sum_inv);
            }

            memo[node as usize][dist_since_inversion as usize] = (min_sum, max_sum);
            (min_sum, max_sum)
        }

        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let n = nums.len();
        let k = k as i64;
        let mut adj = vec![Vec::new(); n];
        let mut memo = vec![vec![(i64::MIN, i64::MIN); k as usize + 1]; n];
        for edge in &edges {
            adj[edge[0] as usize].push(edge[1] as i64);
            adj[edge[1] as usize].push(edge[0] as i64);
        }
        dfs(0, k, -1, &adj, &nums, k, &mut memo).1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::subtree_inversion_sum(
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
            vec![4, -8, -6, 3, 7, -2, 5],
            2
        ),
        27
    );
    assert_eq!(
        Solution::subtree_inversion_sum(vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], vec![-1, 3, -2, 4, -5], 2),
        9
    );
    assert_eq!(Solution::subtree_inversion_sum(vec![vec![0, 1], vec![0, 2]], vec![0, -1, -2], 3), 3);
}
