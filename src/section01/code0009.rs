#![allow(dead_code)]

// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/
// https://leetcode.cn/problems/palindrome-number/
//
// Given an integer x, return true if x is a palindrome, and false otherwise.
//
// Example 1:
//
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
// Example 2:
//
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
// Example 3:
//
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
// Constraints:
//
// - -2^31 <= x <= 2^31 - 1
//
// Follow up: Could you solve it without converting the integer to a string?
//

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();
        x.chars().rev().eq(x.chars())
    }
}

#[test]
fn test() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121));
    assert!(!Solution::is_palindrome(10));
}
