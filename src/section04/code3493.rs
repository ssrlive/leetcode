#![allow(dead_code)]

// 3493. Properties Graph
// https://leetcode.com/problems/properties-graph
// https://leetcode.cn/problems/properties-graph
//
// Medium
//
// You are given a 2D integer array properties having dimensions n x m and an integer k.
//
// Define a function intersect(a, b) that returns the number of distinct integers common to both arrays a and b.
//
// Construct an undirected graph where each index i corresponds to properties[i]. There is an edge between node i and
// node j if and only if intersect(properties[i], properties[j]) >= k, where i and j are in the range [0, n - 1] and i != j.
//
// Return the number of connected components in the resulting graph.
//
// Example 1:
//
// Input: properties = [[1,2],[1,1],[3,4],[4,5],[5,6],[7,7]], k = 1
//
// Output: 3
//
// Explanation:
//
// The graph formed has 3 connected components:
//
// Example 2:
//
// Input: properties = [[1,2,3],[2,3,4],[4,3,5]], k = 2
//
// Output: 1
//
// Explanation:
//
// The graph formed has 1 connected component:
//
// Example 3:
//
// Input: properties = [[1,1],[1,1]], k = 2
//
// Output: 2
//
// Explanation:
//
// intersect(properties[0], properties[1]) = 1, which is less than k.
// This means there is no edge between properties[0] and properties[1] in the graph.
//
// Constraints:
//
//     1 <= n == properties.length <= 100
//     1 <= m == properties[i].length <= 100
//     1 <= properties[i][j] <= 100
//     1 <= k <= m
//

struct Solution;

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        fn dfs(i: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
            if visited[i] {
                return false;
            }
            visited[i] = true;
            for &j in &adj[i] {
                dfs(j, adj, visited);
            }
            true
        }

        let n = properties.len();
        let mut adj = vec![vec![]; n];
        let mut visited = vec![false; n];
        let properties = properties
            .iter()
            .map(|p| p.iter().fold(0u128, |acc, &x| acc | (1 << x)))
            .collect::<Vec<_>>();

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                };
                if (properties[i] & properties[j]).count_ones() >= k as _ {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        (0..n).filter(|&i| dfs(i, &adj, &mut visited)).count() as _
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::number_of_components(vec![vec![1, 2], vec![1, 1], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]], 1),
        3
    );
    assert_eq!(
        Solution::number_of_components(vec![vec![1, 2, 3], vec![2, 3, 4], vec![4, 3, 5]], 2),
        1
    );
    assert_eq!(Solution::number_of_components(vec![vec![1, 1], vec![1, 1]], 2), 2);
}
