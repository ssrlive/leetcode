#![allow(dead_code)]

// 2959. Number of Possible Sets of Closing Branches
// https://leetcode.com/problems/number-of-possible-sets-of-closing-branches/
// https://leetcode.cn/problems/number-of-possible-sets-of-closing-branches/
//
// Hard
//
// There is a company with n branches across the country, some of which are connected by roads.
// Initially, all branches are reachable from each other by traveling some roads.
//
// The company has realized that they are spending an excessive amount of time traveling between their branches.
// As a result, they have decided to close down some of these branches (possibly none).
// However, they want to ensure that the remaining branches have a distance of at most maxDistance from each other.
//
// The distance between two branches is the minimum total traveled length needed to reach one branch from another.
//
// You are given integers n, maxDistance, and a 0-indexed 2D array roads,
// where roads[i] = [ui, vi, wi] represents the undirected road between branches ui and vi with length wi.
//
// Return the number of possible sets of closing branches, so that any branch has a distance of at most maxDistance from any other.
//
// Note that, after closing a branch, the company will no longer have access to any roads connected to it.
//
// Note that, multiple roads are allowed.
//
// Example 1:
//
// Input: n = 3, maxDistance = 5, roads = [[0,1,2],[1,2,10],[0,2,10]]
// Output: 5
// Explanation: The possible sets of closing branches are:
// - The set [2], after closing, active branches are [0,1] and they are reachable to each other within distance 2.
// - The set [0,1], after closing, the active branch is [2].
// - The set [1,2], after closing, the active branch is [0].
// - The set [0,2], after closing, the active branch is [1].
// - The set [0,1,2], after closing, there are no active branches.
// It can be proven, that there are only 5 possible sets of closing branches.
//
// Example 2:
//
// Input: n = 3, maxDistance = 5, roads = [[0,1,20],[0,1,10],[1,2,2],[0,2,2]]
// Output: 7
// Explanation: The possible sets of closing branches are:
// - The set [], after closing, active branches are [0,1,2] and they are reachable to each other within distance 4.
// - The set [0], after closing, active branches are [1,2] and they are reachable to each other within distance 2.
// - The set [1], after closing, active branches are [0,2] and they are reachable to each other within distance 2.
// - The set [0,1], after closing, the active branch is [2].
// - The set [1,2], after closing, the active branch is [0].
// - The set [0,2], after closing, the active branch is [1].
// - The set [0,1,2], after closing, there are no active branches.
// It can be proven, that there are only 7 possible sets of closing branches.
//
// Example 3:
//
// Input: n = 1, maxDistance = 10, roads = []
// Output: 2
// Explanation: The possible sets of closing branches are:
// - The set [], after closing, the active branch is [0].
// - The set [0], after closing, there are no active branches.
// It can be proven, that there are only 2 possible sets of closing branches.
//
// Constraints:
//
//     1 <= n <= 10
//     1 <= maxDistance <= 10^5
//     0 <= roads.length <= 1000
//     roads[i].length == 3
//     0 <= u[i], v[i] <= n - 1
//     u[i] != v[i]
//     1 <= w[i] <= 1000
//     All branches are reachable from each other by traveling some roads.
//

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut d = vec![vec![100000; n as usize]; n as usize];
        for r in &roads {
            d[r[0] as usize][r[1] as usize] = r[2].min(d[r[0] as usize][r[1] as usize]);
            d[r[1] as usize][r[0] as usize] = r[2].min(d[r[1] as usize][r[0] as usize]);
        }
        let comb = 1 << n;
        let mut res = 1;
        for mask in 1..comb {
            if Self::get_max_distance(mask, n, &d) <= max_distance {
                res += 1;
            }
        }
        res
    }

    fn get_max_distance(mask: i32, n: i32, d: &[Vec<i32>]) -> i32 {
        let mut res = 0;
        let mut d = d.to_owned();
        for k in 0..n {
            if mask & (1 << k) != 0 {
                for i in 0..n {
                    if i != k && mask & (1 << i) != 0 {
                        for j in 0..n {
                            if j != i && mask & (1 << j) != 0 {
                                d[i as usize][j as usize] =
                                    d[i as usize][j as usize].min(d[i as usize][k as usize] + d[k as usize][j as usize]);
                            }
                        }
                    }
                }
            }
        }
        for i in 0..n {
            if mask & (1 << i) != 0 {
                for j in (i + 1)..n {
                    if mask & (1 << j) != 0 {
                        res = res.max(d[i as usize][j as usize]);
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::number_of_sets(3, 5, vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]]),
        5
    );
    assert_eq!(
        Solution::number_of_sets(3, 5, vec![vec![0, 1, 20], vec![0, 1, 10], vec![1, 2, 2], vec![0, 2, 2]]),
        7
    );
    assert_eq!(Solution::number_of_sets(1, 10, vec![]), 2);
}
