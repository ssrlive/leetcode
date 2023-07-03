#![allow(dead_code)]

// 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance
// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
// https://leetcode.cn/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
//
// Medium
//
// There are n cities numbered from 0 to n-1. Given the array edges where edges[i] = [fromi, toi, weighti] represents
// a bidirectional and weighted edge between cities fromi and toi, and given the integer distanceThreshold.
//
// Return the city with the smallest number of cities that are reachable through some path and whose distance is at most distanceThreshold,
// If there are multiple such cities, return the city with the greatest number.
//
// Notice that the distance of a path connecting cities i and j is equal to the sum of the edges' weights along that path.
//
// Example 1:
//
// Input: n = 4, edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]], distanceThreshold = 4
// Output: 3
// Explanation: The figure above describes the graph.
// The neighboring cities at a distanceThreshold = 4 for each city are:
// City 0 -> [City 1, City 2]
// City 1 -> [City 0, City 2, City 3]
// City 2 -> [City 0, City 1, City 3]
// City 3 -> [City 1, City 2]
// Cities 0 and 3 have 2 neighboring cities at a distanceThreshold = 4, but we have to return city 3 since it has the greatest number.
//
// Example 2:
//
// Input: n = 5, edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]], distanceThreshold = 2
// Output: 0
// Explanation: The figure above describes the graph.
// The neighboring cities at a distanceThreshold = 2 for each city are:
// City 0 -> [City 1]
// City 1 -> [City 0, City 4]
// City 2 -> [City 3, City 4]
// City 3 -> [City 2, City 4]
// City 4 -> [City 1, City 2, City 3]
// The city 0 has 1 neighboring city at a distanceThreshold = 2.
//
// Constraints:
//
// -    2 <= n <= 100
// -    1 <= edges.length <= n * (n - 1) / 2
// -    edges[i].length == 3
// -    0 <= fromi < toi < n
// -    1 <= weighti, distanceThreshold <= 10^4
// -    All pairs (fromi, toi) are distinct.
//

struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut graph = vec![vec![std::i32::MAX; n as usize]; n as usize];
        for (i, item) in graph.iter_mut().enumerate() {
            item[i] = 0;
        }
        for edge in edges {
            graph[edge[0] as usize][edge[1] as usize] = edge[2];
            graph[edge[1] as usize][edge[0] as usize] = edge[2];
        }
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    if graph[i][k] != std::i32::MAX && graph[k][j] != std::i32::MAX {
                        graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                    }
                }
            }
        }
        let mut min = std::i32::MAX;
        let mut res = 0;
        for (i, item) in graph.iter().enumerate() {
            let mut count = 0;
            for &item2 in item.iter() {
                if item2 <= distance_threshold {
                    count += 1;
                }
            }
            if count <= min {
                min = count;
                res = i;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]], 4, 3),
        (
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1],
            ],
            2,
            0,
        ),
    ];
    for (n, edges, distance_threshold, expect) in cases {
        assert_eq!(Solution::find_the_city(n, edges, distance_threshold), expect);
    }
}
