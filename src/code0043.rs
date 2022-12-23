#![allow(dead_code)]

// 43. Multiply Strings
// https://leetcode.com/problems/multiply-strings/
// https://leetcode.cn/problems/multiply-strings/
//
// Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
//
// Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
//
// Example 1:
//
// Input: num1 = "2", num2 = "3"
// Output: "6"
//
// Example 2:
//
// Input: num1 = "123", num2 = "456"
// Output: "56088"
//
// Constraints:
//
// - 1 <= num1.length, num2.length <= 200
// - num1 and num2 consist of digits only.
// - Both num1 and num2 do not contain any leading zero, except the number 0 itself.
//

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let len1 = num1.len();
        let len2 = num2.len();
        let digits = {
            let mut digits = vec![0; len1 + len2];
            for (idx1, ch1) in num1.chars().rev().enumerate() {
                for (idx2, ch2) in num2.chars().rev().enumerate() {
                    let n1 = ch1 as i32 - '0' as i32;
                    let n2 = ch2 as i32 - '0' as i32;
                    let res = n1 * n2 + digits[idx1 + idx2];
                    digits[idx1 + idx2] = res % 10;
                    digits[idx1 + idx2 + 1] += res / 10;
                }
            }
            while digits.len() > 1 && digits.last() == Some(&0) {
                digits.pop();
            }
            digits
        };
        digits.into_iter().rev().map(|d| d.to_string()).collect::<String>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
    assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088");
}
