#![allow(dead_code)]

// 2440. Create Components With Same Value
// https://leetcode.com/problems/create-components-with-same-value/
// https://leetcode.cn/problems/create-components-with-same-value/
//
// There is an undirected tree with n nodes labeled from 0 to n - 1.
//
// You are given a 0-indexed integer array nums of length n where nums[i] represents the value of the ith node. You are also given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// You are allowed to delete some edges, splitting the tree into multiple connected components. Let the value of a component be the sum of all nums[i] for which node i is in the component.
//
// Return the maximum number of edges you can delete, such that every connected component in the tree has the same value.
//
// Example 1:
//
// Input: nums = [6,2,2,2,6], edges = [[0,1],[1,2],[1,3],[3,4]]
// Output: 2
// Explanation: The above figure shows how we can delete the edges [0,1] and [3,4]. The created components are nodes [0], [1,2,3] and [4]. The sum of the values in each component equals 6. It can be proven that no better deletion exists, so the answer is 2.
//
// Example 2:
//
// Input: nums = [2], edges = []
// Output: 0
// Explanation: There are no edges to be deleted.
//
// Constraints:
//
// - 1 <= n <= 2 * 10^4
// - nums.length == n
// - 1 <= nums[i] <= 50
// - edges.length == n - 1
// - edges[i].length == 2
// - 0 <= edges[i][0], edges[i][1] <= n - 1
// - edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut graph = vec![vec![]; n];

        for e in edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let sum: i32 = nums.iter().sum();

        for a in 1..sum as usize {
            let a = a as i32;
            if sum % a != 0 {
                continue;
            }

            let mut flag = vec![0; n];
            flag[0] = 1;
            let mut rem = 0;
            if Self::check(&graph, &mut flag, 0, a, &mut rem, &nums) {
                return sum / a - 1;
            }
        }

        0
    }

    fn check(graph: &Vec<Vec<usize>>, flag: &mut Vec<i32>, u: usize, amt: i32, rem: &mut i32, nums: &Vec<i32>) -> bool {
        *rem = nums[u];
        for v in &graph[u] {
            if flag[*v] == 1 {
                continue;
            }

            flag[*v] = 1;
            let mut t = 0;
            if !Self::check(graph, flag, *v, amt, &mut t, nums) {
                return false;
            }
            *rem += t;
        }
        if *rem > amt {
            return false;
        }
        if *rem == amt {
            *rem = 0;
        }

        true
    }
}

#[test]
fn test() {
    let nums = vec![6, 2, 2, 2, 6];
    let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
    assert_eq!(Solution::component_value(nums, edges), 2);

    let nums = vec![2];
    let edges = vec![];
    assert_eq!(Solution::component_value(nums, edges), 0);
}
