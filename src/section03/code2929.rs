#![allow(dead_code)]

// 2929. Distribute Candies Among Children II
// https://leetcode.com/problems/distribute-candies-among-children-ii/
// https://leetcode.cn/problems/distribute-candies-among-children-ii/
//
// Medium
//
// You are given two positive integers n and limit.
//
// Return the total number of ways to distribute n candies among 3 children such that no child gets more than limit candies.
//
// Example 1:
//
// Input: n = 5, limit = 2
// Output: 3
// Explanation: There are 3 ways to distribute 5 candies such that no child gets more than 2 candies:
// (1, 2, 2), (2, 1, 2) and (2, 2, 1).
//
// Example 2:
//
// Input: n = 3, limit = 3
// Output: 10
// Explanation: There are 10 ways to distribute 3 candies such that no child gets more than 3 candies:
// (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0, 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
//
// Constraints:
//
//     1 <= n <= 10^6
//     1 <= limit <= 10^6
//

struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut res = 0;
        let first_min = 0.max(n - 2 * limit);
        let first_max = limit.min(n);
        for i in first_min..=first_max {
            let second_min = 0.max(n - i - limit);
            let second_max = limit.min(n - i);
            res += (second_max - second_min + 1) as i64;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::distribute_candies(5, 2), 3);
    assert_eq!(Solution::distribute_candies(3, 3), 10);
}
