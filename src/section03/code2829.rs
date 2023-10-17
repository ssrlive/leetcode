#![allow(dead_code)]

// 2829. Determine the Minimum Sum of a k-avoiding Array
// https://leetcode.com/problems/determine-the-minimum-sum-of-a-k-avoiding-array/
// https://leetcode.cn/problems/determine-the-minimum-sum-of-a-k-avoiding-array/
//
// Medium
//
// You are given two integers, n and k.
//
// An array of distinct positive integers is called a k-avoiding array if there does not exist any pair of distinct elements that sum to k.
//
// Return the minimum possible sum of a k-avoiding array of length n.
//
// Example 1:
//
// Input: n = 5, k = 4
// Output: 18
// Explanation: Consider the k-avoiding array [1,2,4,5,6], which has a sum of 18.
// It can be proven that there is no k-avoiding array with a sum less than 18.
//
// Example 2:
//
// Input: n = 2, k = 6
// Output: 3
// Explanation: We can construct the array [1,2], which has a sum of 3.
// It can be proven that there is no k-avoiding array with a sum less than 3.
//
// Constraints:
//
// 1 <= n, k <= 50
//

struct Solution;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut i = 1;
        let mut s = std::collections::HashSet::new();

        while s.len() < n as usize {
            if !s.contains(&(k - i)) {
                s.insert(i);
            }
            i += 1;
        }

        s.into_iter().sum::<_>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_sum(5, 4), 18);
    assert_eq!(Solution::minimum_sum(2, 6), 3);
}
