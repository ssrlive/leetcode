#![allow(dead_code)]

// 1191. K-Concatenation Maximum Sum
// https://leetcode.com/problems/k-concatenation-maximum-sum/
// https://leetcode.cn/problems/k-concatenation-maximum-sum/
//
// Given an integer array arr and an integer k, modify the array by repeating it k times.
//
// For example, if arr = [1, 2] and k = 3 then the modified array will be [1, 2, 1, 2, 1, 2].
//
// Return the maximum sub-array sum in the modified array. Note that the length of the sub-array can be 0 and its sum in that case is 0.
//
// As the answer can be very large, return the answer modulo 109 + 7.
//
// Example 1:
//
// Input: arr = [1,2], k = 3
// Output: 9
//
// Example 2:
//
// Input: arr = [1,-2,1], k = 5
// Output: 2
//
// Example 3:
//
// Input: arr = [-1,-2], k = 7
// Output: 0
//
// Constraints:
//
// - 1 <= arr.length <= 10^5
// - 1 <= k <= 10^5
// - -10^4 <= arr[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        use std::cmp::{max, min};
        let k = k as i64;
        let arr = arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut m_sum = 0;
        let sz = arr.len() as i64;
        let mut sum = 0;
        for i in 0..min(2, k) * sz {
            sum = max(sum + arr[(i % sz) as usize], arr[(i % sz) as usize]);
            m_sum = max(m_sum, sum);
        }
        ((m_sum + max(0, arr.iter().sum::<i64>()) * max(0, k - 2)) % 1000000007) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
}
