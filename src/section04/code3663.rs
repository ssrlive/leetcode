#![allow(dead_code)]

// 3663. Find The Least Frequent Digit
// https://leetcode.com/problems/find-the-least-frequent-digit/
// https://leetcode.cn/problems/find-the-least-frequent-digit/
//
// Easy
//
// Given an integer n, find the digit that occurs least frequently in its decimal representation. If multiple digits have the same frequency, choose the smallest digit.
//
// Return the chosen digit as an integer.
//
// The frequency of a digit x is the number of times it appears in the decimal representation of n.
//
// Example 1:
//
// Input: n = 1553322
//
// Output: 1
//
// Explanation:
//
// The least frequent digit in n is 1, which appears only once. All other digits appear twice.
//
// Example 2:
//
// Input: n = 723344511
//
// Output: 2
//
// Explanation:
//
// The least frequent digits in n are 7, 2, and 5; each appears only once.
//
// Constraints:
//
// 1 <= n <= 2^31​​​​​​​ - 1
//

struct Solution;

impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut freq = [0; 10];
        let mut n = n;

        while n > 0 {
            let d = (n % 10) as usize;
            freq[d] += 1;
            n /= 10;
        }

        let mut min_d = i32::MAX;
        let mut min_f = i32::MAX;
        for (i, &freq_i) in freq.iter().enumerate() {
            let f = freq_i;
            if f == 0 {
                continue;
            }
            if f < min_f {
                min_f = f;
                min_d = i as i32;
            }
        }

        min_d
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_least_frequent_digit(1553322), 1);
    assert_eq!(Solution::get_least_frequent_digit(723344511), 2);
}
