#![allow(dead_code)]

// 556. Next Greater Element III
// https://leetcode.com/problems/next-greater-element-iii/
// https://leetcode.cn/problems/next-greater-element-iii/
//
// Given a positive integer n, find the smallest integer which has exactly the same digits existing
// in the integer n and is greater in value than n. If no such positive integer exists, return -1.
//
// Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.
//
// Example 1:
//
// Input: n = 12
// Output: 21
//
// Example 2:
//
// Input: n = 21
// Output: -1
//
// Constraints:
//
// - 1 <= n <= 2^31 - 1
//
// Follow up: Could you solve it without modifying the input n?
//

struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = Vec::new();
        let mut n = n as i64;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let mut i = digits.len() - 1;
        while i > 0 && digits[i - 1] >= digits[i] {
            i -= 1;
        }
        if i == 0 {
            return -1;
        }
        let mut j = digits.len() - 1;
        while j > i && digits[j] <= digits[i - 1] {
            j -= 1;
        }
        digits.swap(i - 1, j);
        digits[i..].reverse();
        let mut result = 0;
        for &digit in digits.iter() {
            result = result * 10 + digit;
        }
        if result > i32::MAX as i64 { -1 } else { result as i32 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_greater_element(12), 21);
    assert_eq!(Solution::next_greater_element(21), -1);
    assert_eq!(Solution::next_greater_element(123456789), 123456798);
    assert_eq!(Solution::next_greater_element(1999999999), -1);
}
