#![allow(dead_code)]

// 504. Base 7
// https://leetcode.com/problems/base-7/
//
// Given an integer num, return a string of its base 7 representation.
//
// Example 1:
//
// Input: num = 100
// Output: "202"
//
// Example 2:
//
// Input: num = -7
// Output: "-10"
//
// Constraints:
//
// -10^7 <= num <= 10^7

struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut num = num;
        let mut result = String::new();
        let mut is_negative = false;

        if num == 0 {
            return "0".to_string();
        }

        if num < 0 {
            is_negative = true;
            num = -num;
        }

        while num > 0 {
            result.push(((num % 7) as u8 + b'0') as char);
            num /= 7;
        }

        if is_negative {
            result.push('-');
        }

        result.chars().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::convert_to_base7(100), "202");
    assert_eq!(Solution::convert_to_base7(-7), "-10");
}
