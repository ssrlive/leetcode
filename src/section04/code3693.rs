#![allow(dead_code)]

// 3693. Climbing Stairs II
// https://leetcode.com/problems/climbing-stairs-ii/
// https://leetcode.cn/problems/climbing-stairs-ii/
//
// Medium
//
// You are climbing a staircase with n + 1 steps, numbered from 0 to n.
//
// You are also given a 1-indexed integer array costs of length n, where costs[i] is the cost of step i.
//
// From step i, you can jump only to step i + 1, i + 2, or i + 3. The cost of jumping from step i to step j is defined as: costs[j] + (j - i)2
//
// You start from step 0 with cost = 0.
//
// Return the minimum total cost to reach step n.
//
// Example 1:
//
// Input: n = 4, costs = [1,2,3,4]
//
// Output: 13
//
// Explanation:
//
// One optimal path is 0 → 1 → 2 → 4
//
// Jump	Cost Calculation	Cost
// 0 → 1	costs[1] + (1 - 0)2 = 1 + 1	2
// 1 → 2	costs[2] + (2 - 1)2 = 2 + 1	3
// 2 → 4	costs[4] + (4 - 2)2 = 4 + 4	8
// Thus, the minimum total cost is 2 + 3 + 8 = 13
//
// Example 2:
//
// Input: n = 4, costs = [5,1,6,2]
//
// Output: 11
//
// Explanation:
//
// One optimal path is 0 → 2 → 4
//
// Jump	Cost Calculation	Cost
// 0 → 2	costs[2] + (2 - 0)2 = 1 + 4	5
// 2 → 4	costs[4] + (4 - 2)2 = 2 + 4	6
// Thus, the minimum total cost is 5 + 6 = 11
//
// Example 3:
//
// Input: n = 3, costs = [9,8,3]
//
// Output: 12
//
// Explanation:
//
// The optimal path is 0 → 3 with total cost = costs[3] + (3 - 0)2 = 3 + 9 = 12
//
// Constraints:
//
// 1 <= n == costs.length <= 10^5​​​​​​​
// 1 <= costs[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn climb_stairs(_n: i32, costs: Vec<i32>) -> i32 {
        costs
            .into_iter()
            .fold((0, 0, 0), |(a, b, c), cost| (b, c, cost + (a + 9).min(b + 4).min(c + 1)))
            .2
    }
}

#[test]
fn test() {
    let n = 4;
    let costs = vec![1, 2, 3, 4];
    assert_eq!(Solution::climb_stairs(n, costs), 13);

    let n = 4;
    let costs = vec![5, 1, 6, 2];
    assert_eq!(Solution::climb_stairs(n, costs), 11);

    let n = 3;
    let costs = vec![9, 8, 3];
    assert_eq!(Solution::climb_stairs(n, costs), 12);
}
