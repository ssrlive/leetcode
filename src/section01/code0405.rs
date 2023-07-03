#![allow(dead_code)]

// 405. Convert a Number to Hexadecimal
// https://leetcode.com/problems/convert-a-number-to-hexadecimal/
// https://leetcode.cn/problems/convert-a-number-to-hexadecimal/
//
// Given an integer num, return a string representing its hexadecimal representation.
// For negative integers, two’s complement method is used.
//
// All the letters in the answer string should be lowercase characters, and there should
// not be any leading zeros in the answer except for the zero itself.
//
// Note: You are not allowed to use any built-in library method to directly solve this problem.
//
// Example 1:
//
// Input: num = 26
// Output: "1a"
//
// Example 2:
//
// Input: num = -1
// Output: "ffffffff"
//
// Constraints:
//
// -2^31 <= num <= 2^31 - 1
//
// Follow up: Could you solve it without using any loop / recursion?
//

struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut n = num as u32;
        let mut res = Vec::new();
        let map = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

        while n > 0 {
            res.push(map[(n % 16) as usize]);
            n /= 16;
        }
        res.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::to_hex(26), "1a");
    assert_eq!(Solution::to_hex(-1), "ffffffff");
}
