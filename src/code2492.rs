#![allow(dead_code)]

// 2492. Minimum Score of a Path Between Two Cities
// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/
// https://leetcode.cn/problems/minimum-score-of-a-path-between-two-cities/
//
// You are given a positive integer n representing n cities numbered from 1 to n. You are also given a 2D array roads where
// roads[i] = [ai, bi, distancei] indicates that there is a bidirectional road between cities ai and bi with a distance equal to distancei.
// The cities graph is not necessarily connected.
//
// The score of a path between two cities is defined as the minimum distance of a road in this path.
//
// Return the minimum possible score of a path between cities 1 and n.
//
// Note:
//
// A path is a sequence of roads between two cities.
// It is allowed for a path to contain the same road multiple times, and you can visit cities 1 and n multiple times along the path.
// The test cases are generated such that there is at least one path between 1 and n.
//
// Example 1:
//
// Input: n = 4, roads = [[1,2,9],[2,3,6],[2,4,5],[1,4,7]]
// Output: 5
// Explanation: The path from city 1 to 4 with the minimum score is: 1 -> 2 -> 4. The score of this path is min(9,5) = 5.
// It can be shown that no other path has less score.
//
// Example 2:
//
// Input: n = 4, roads = [[1,2,2],[1,3,4],[3,4,7]]
// Output: 2
// Explanation: The path from city 1 to 4 with the minimum score is: 1 -> 2 -> 1 -> 3 -> 4. The score of this path is min(2,2,4,7) = 2.
//
// Constraints:
//
// - 2 <= n <= 10^5
// - 1 <= roads.length <= 10^5
// - roads[i].length == 3
// - 1 <= ai, bi <= n
// - ai != bi
// - 1 <= distancei <= 10^4
// - There are no repeated edges.
// - There is at least one path between 1 and n.
//

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ans = i32::MAX;
        let mut neighbors = vec![vec![]; n + 1];
        for road in &roads {
            neighbors[road[0] as usize].push((road[1] as usize, road[2]));
            neighbors[road[1] as usize].push((road[0] as usize, road[2]));
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(1);
        queue.push_back(n);
        let mut visited = vec![false; n + 1];
        visited[1] = true;
        visited[n] = true;
        while let Some(i) = queue.pop_front() {
            for &(j, dist) in &neighbors[i] {
                if !visited[j] {
                    queue.push_back(j);
                    visited[j] = true;
                }
                ans = ans.min(dist);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let n = 4;
    let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
    assert_eq!(Solution::min_score(n, roads), 5);
    let n = 4;
    let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
    assert_eq!(Solution::min_score(n, roads), 2);
}
