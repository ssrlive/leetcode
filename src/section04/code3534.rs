#![allow(dead_code)]

// 3534. Path Existence Queries in a Graph II
// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii/
// https://leetcode.cn/problems/path-existence-queries-in-a-graph-ii/
//
// Hard
//
// You are given an integer n representing the number of nodes in a graph, labeled from 0 to n - 1.
//
// You are also given an integer array nums of length n and an integer maxDiff.
//
// An undirected edge exists between nodes i and j if the absolute difference between nums[i]
// and nums[j] is at most maxDiff (i.e., |nums[i] - nums[j]| <= maxDiff).
//
// You are also given a 2D integer array queries. For each queries[i] = [ui, vi], find the minimum
// distance between nodes ui and vi. If no path exists between the two nodes, return -1 for that query.
//
// Return an array answer, where answer[i] is the result of the ith query.
//
// Note: The edges between the nodes are unweighted.
//
// Example 1:
//
// Input: n = 5, nums = [1,8,3,4,2], maxDiff = 3, queries = [[0,3],[2,4]]
//
// Output: [1,1]
//
// Explanation:
//
// The resulting graph is:
//
// Query	Shortest Path	Minimum Distance
// [0, 3]	0 → 3	1
// [2, 4]	2 → 4	1
//
// Thus, the output is [1, 1].
//
// Example 2:
//
// Input: n = 5, nums = [5,3,1,9,10], maxDiff = 2, queries = [[0,1],[0,2],[2,3],[4,3]]
//
// Output: [1,2,-1,1]
//
// Explanation:
//
// The resulting graph is:
//
// Query	Shortest Path	Minimum Distance
// [0, 1]	0 → 1	1
// [0, 2]	0 → 1 → 2	2
// [2, 3]	None	-1
// [4, 3]	3 → 4	1
//
// Thus, the output is [1, 2, -1, 1].
//
// Example 3:
//
// Input: n = 3, nums = [3,6,1], maxDiff = 1, queries = [[0,0],[0,1],[1,2]]
//
// Output: [0,-1,-1]
//
// Explanation:
//
// There are no edges between any two nodes because:
//
//     Nodes 0 and 1: |nums[0] - nums[1]| = |3 - 6| = 3 > 1
//     Nodes 0 and 2: |nums[0] - nums[2]| = |3 - 1| = 2 > 1
//     Nodes 1 and 2: |nums[1] - nums[2]| = |6 - 1| = 5 > 1
//
// Thus, no node can reach any other node, and the output is [0, -1, -1].
//
// Constraints:
//
//     1 <= n == nums.length <= 10^5
//     0 <= nums[i] <= 10^5
//     0 <= maxDiff <= 10^5
//     1 <= queries.length <= 10^5
//     queries[i] == [ui, vi]
//     0 <= ui, vi < n
//

struct Solution;

impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // Preprocessing
        let mut num_with_ind: Vec<(i32, i32)> = nums.iter().copied().enumerate().map(|(i, v)| (v, i as i32)).collect();
        num_with_ind.sort_unstable();

        // Old ind => NewInd ind
        let mut new_ind = vec![0; n as usize];
        for i in 0..n {
            new_ind[num_with_ind[i as usize].1 as usize] = i;
        }

        // All use new index to operation
        let mut group = vec![0; n as usize];
        let mut g = 0;
        for i in 1..n {
            if num_with_ind[i as usize].0 - num_with_ind[(i - 1) as usize].0 > max_diff {
                g += 1;
            }
            group[i as usize] = g;
        }

        // Find the each node can go to where after 2^row steps, 2^16 = 65536
        let mut dp = vec![vec![0; n as usize]; 17];
        let mut steps = [0; 17];
        steps[0] = 1;
        let mut r = 0;
        for i in 0..n {
            while r < n && num_with_ind[r as usize].0 - num_with_ind[i as usize].0 <= max_diff {
                r += 1;
            }
            dp[0][i as usize] = r - 1;
        }
        for row in 1..17 {
            steps[row] = steps[row - 1] * 2;
            for i in 0..n {
                dp[row][i as usize] = dp[row - 1][dp[row - 1][i as usize] as usize];
            }
        }

        // Finally, get result
        let mut result: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            if group[new_ind[q[0] as usize] as usize] != group[new_ind[q[1] as usize] as usize] {
                result.push(-1);
            } else {
                let (mut ind1, mut ind2) = (new_ind[q[0] as usize], new_ind[q[1] as usize]);
                if ind2 < ind1 {
                    std::mem::swap(&mut ind1, &mut ind2);
                } // always ind1 < ind2
                if ind1 == ind2 {
                    result.push(0);
                    continue;
                }
                let mut step = 0;
                let mut cur = ind1;
                for i in (0..=16).rev() {
                    // try to add 2^i steps
                    if dp[i][cur as usize] < ind2 {
                        cur = dp[i][cur as usize];
                        step += steps[i];
                    }
                }
                // Explain why need to add 1 in approach part fourth paragraph.
                result.push(step + 1);
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::path_existence_queries(5, vec![1, 8, 3, 4, 2], 3, vec![vec![0, 3], vec![2, 4]]),
        vec![1, 1]
    );
    assert_eq!(
        Solution::path_existence_queries(5, vec![5, 3, 1, 9, 10], 2, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]]),
        vec![1, 2, -1, 1]
    );
    assert_eq!(
        Solution::path_existence_queries(3, vec![3, 6, 1], 1, vec![vec![0, 0], vec![0, 1], vec![1, 2]]),
        vec![0, -1, -1]
    );
}
