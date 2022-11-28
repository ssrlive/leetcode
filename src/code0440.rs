#![allow(dead_code)]

// 440. K-th Smallest in Lexicographical Order
// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
//
// Given an integer n and an integer k, return the kth lexicographically smallest integer in the range [1, n].
//
// Example 1:
//
// Input: n = 13, k = 2
// Output: 10
// Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]. The second smallest number is 10.
//
// Example 2:
//
// Input: n = 1, k = 1
// Output: 1
//
// Constraints:
//
// 1 <= k <= n <= 10^9

struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        pub fn find_kth_number(n: i64, k: i64) -> i64 {
            let mut k = k;
            let mut answer = 1;
            while k > 1 {
                let mut count = 0;
                let mut left = answer;
                let mut right = answer + 1;
                while left <= n {
                    count += std::cmp::min(n + 1, right) - left;
                    left *= 10;
                    right *= 10;
                }
                if count < k {
                    answer += 1;
                    k -= count;
                } else {
                    answer *= 10;
                    k -= 1;
                }
            }
            answer
        }
        find_kth_number(n as i64, k as i64) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_number(13, 2), 10);
    assert_eq!(Solution::find_kth_number(1, 1), 1);
}
