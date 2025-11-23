#![allow(dead_code)]

// 3710. Maximum Partition Factor
// https://leetcode.com/problems/maximum-partition-factor/
// https://leetcode.cn/problems/maximum-partition-factor/
//
// Hard
//
// You are given a 2D integer array points, where points[i] = [xi, yi] represents the coordinates of the ith point on the Cartesian plane.
//
// The Manhattan distance between two points points[i] = [xi, yi] and points[j] = [xj, yj] is |xi - xj| + |yi - yj|.
//
// Split the n points into exactly two non-empty groups. The partition factor of a split is the minimum
// Manhattan distance among all unordered pairs of points that lie in the same group.
//
// Return the maximum possible partition factor over all valid splits.
//
// Note: A group of size 1 contributes no intra-group pairs. When n = 2 (both groups size 1),
// there are no intra-group pairs, so define the partition factor as 0.
//
// Example 1:
//
// Input: points = [[0,0],[0,2],[2,0],[2,2]]
//
// Output: 4
//
// Explanation:
//
// We split the points into two groups: {[0, 0], [2, 2]} and {[0, 2], [2, 0]}.
//
// In the first group, the only pair has Manhattan distance |0 - 2| + |0 - 2| = 4.
//
// In the second group, the only pair also has Manhattan distance |0 - 2| + |2 - 0| = 4.
//
// The partition factor of this split is min(4, 4) = 4, which is maximal.
//
// Example 2:
//
// Input: points = [[0,0],[0,1],[10,0]]
//
// Output: 11
//
// Explanation:​​​​​​​
//
// We split the points into two groups: {[0, 1], [10, 0]} and {[0, 0]}.
//
// In the first group, the only pair has Manhattan distance |0 - 10| + |1 - 0| = 11.
//
// The second group is a singleton, so it contributes no pairs.
//
// The partition factor of this split is 11, which is maximal.
//
// Constraints:
//
// 2 <= points.length <= 500
// points[i] = [xi, yi]
// -10^8 <= xi, yi <= 10^8
//

struct Solution;

impl Solution {
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
        fn bfs(node: usize, adj: &[Vec<usize>], vis: &mut [i32]) -> bool {
            let n = adj.len();
            let mut color = vec![-1; n];
            let mut q = std::collections::VecDeque::new();
            q.push_back((node, 0));
            color[node] = 0;
            vis[node] = 1;
            while let Some((x, c)) = q.pop_front() {
                let next_color = 1 - c;
                for &it in &adj[x] {
                    if vis[it] == 1 {
                        if color[it] != next_color {
                            return false;
                        }
                    } else {
                        vis[it] = 1;
                        color[it] = next_color;
                        q.push_back((it, color[it]));
                    }
                }
            }
            true
        }

        let n = points.len();
        let mut l = 0;
        let mut r = 1_000_000_000;
        let dist = |i: usize, j: usize| -> i32 { (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs() };
        let even_cycle_check = |adj: &[Vec<usize>], vis: &mut Vec<i32>| {
            for i in 0..n {
                if vis[i] == 0 && !bfs(i, adj, vis) {
                    return false;
                }
            }
            true
        };
        let check = |mini: i32| -> bool {
            let mut adj = vec![vec![]; n];
            let mut vis = vec![0; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    if dist(i, j) < mini {
                        adj[i].push(j);
                        adj[j].push(i);
                    }
                }
            }
            even_cycle_check(&adj, &mut vis)
        };
        while l <= r {
            let mid = (r - l) / 2 + l;
            if check(mid) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        if r >= 1_000_000_000 { 0 } else { r }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::max_partition_factor(vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]]),
        4
    );
    assert_eq!(Solution::max_partition_factor(vec![vec![0, 0], vec![0, 1], vec![10, 0]]), 11);
}
