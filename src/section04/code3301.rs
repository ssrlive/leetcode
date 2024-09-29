#![allow(dead_code)]

// 3301. Maximize the Total Height of Unique Towers
// https://leetcode.com/problems/maximize-the-total-height-of-unique-towers/
// https://leetcode.cn/problems/maximize-the-total-height-of-unique-towers/
//
// Medium
//
// You are given an array maximumHeight, where maximumHeight[i] denotes the maximum height the ith tower can be assigned.
//
// Your task is to assign a height to each tower so that:
//
//     The height of the ith tower is a positive integer and does not exceed maximumHeight[i].
//     No two towers have the same height.
//
// Return the maximum possible total sum of the tower heights. If it's not possible to assign heights, return -1.
//
// Example 1:
//
// Input: maximumHeight = [2,3,4,3]
//
// Output: 10
//
// Explanation:
//
// We can assign heights in the following way: [1, 2, 4, 3].
//
// Example 2:
//
// Input: maximumHeight = [15,10]
//
// Output: 25
//
// Explanation:
//
// We can assign heights in the following way: [15, 10].
//
// Example 3:
//
// Input: maximumHeight = [2,2,1]
//
// Output: -1
//
// Explanation:
//
// It's impossible to assign positive heights to each index so that no two towers have the same height.
//
// Constraints:
//
//     1 <= maximumHeight.length <= 10^5
//     1 <= maximumHeight[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
        let mut maximum_height = maximum_height;
        maximum_height.sort_unstable_by(|a, b| b.cmp(a));
        let mut now = maximum_height[0];
        let mut sum = 0_i64;
        let n = maximum_height.len();
        for &maximum_height_i in maximum_height.iter().take(n) {
            now = now.min(maximum_height_i);
            if now <= 0 {
                return -1;
            }
            sum += now as i64;
            now -= 1;
        }
        sum
    }
}

#[test]
fn test() {
    let maximum_height = vec![2, 3, 4, 3];
    let res = 10;
    assert_eq!(Solution::maximum_total_sum(maximum_height), res);

    let maximum_height = vec![15, 10];
    let res = 25;
    assert_eq!(Solution::maximum_total_sum(maximum_height), res);

    let maximum_height = vec![2, 2, 1];
    let res = -1;
    assert_eq!(Solution::maximum_total_sum(maximum_height), res);
}
