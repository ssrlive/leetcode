#![allow(dead_code)]

// 415. Add Strings
// https://leetcode.com/problems/add-strings/
//
// Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.
//
// You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.
//
// Example 1:
//
// Input: num1 = "11", num2 = "123"
// Output: "134"
//
// Example 2:
//
// Input: num1 = "456", num2 = "77"
// Output: "533"
//
// Example 3:
//
// Input: num1 = "0", num2 = "0"
// Output: "0"
//
// Constraints:
//
// - 1 <= num1.length, num2.length <= 104
// - num1 and num2 consist of only digits.
// - num1 and num2 don't have any leading zeros except for the zero itself.
//
// Follow up: Could you solve it without using any built-in library for handling large integers (such as BigInteger)? Could you solve it without converting the inputs to integer directly?

struct Solution;

impl Solution {
    pub fn add_strings(num1: &str, num2: &str) -> String {
        let mut num1 = num1.chars().collect::<Vec<char>>();
        let mut num2 = num2.chars().collect::<Vec<char>>();
        let mut carry = 0;
        let mut result = String::new();
        while !num1.is_empty() || !num2.is_empty() || carry > 0 {
            let mut sum = carry;
            if let Some(digit) = num1.pop() {
                sum += digit.to_digit(10).unwrap() as i32;
            }
            if let Some(digit) = num2.pop() {
                sum += digit.to_digit(10).unwrap() as i32;
            }
            carry = sum / 10;
            result.push((sum % 10).to_string().chars().next().unwrap());
        }
        result.chars().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::add_strings("11", "123"), "134");
    assert_eq!(Solution::add_strings("456", "77"), "533");
    assert_eq!(Solution::add_strings("0", "0"), "0");
}
