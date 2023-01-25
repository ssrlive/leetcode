#![allow(dead_code)]

// 1304. Find N Unique Integers Sum up to Zero
// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/
// https://leetcode.cn/problems/find-n-unique-integers-sum-up-to-zero/
//
// Easy
//
// Given an integer n, return any array containing n unique integers such that they add up to 0.
//
// Example 1:
//
// Input: n = 5
// Output: [-7,-1,1,3,4]
// Explanation: These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
//
// Example 2:
//
// Input: n = 3
// Output: [-1,0,1]
//
// Example 3:
//
// Input: n = 1
// Output: [0]
//
// Constraints:
//
//     1 <= n <= 1000
//

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        (1..n).chain(std::iter::once(-n * (n - 1) / 2)).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_zero(5), vec![1, 2, 3, 4, -10]);
    assert_eq!(Solution::sum_zero(3), vec![1, 2, -3]);
    assert_eq!(Solution::sum_zero(1), vec![0]);
}
