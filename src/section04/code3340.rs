#![allow(dead_code)]

// 3340. Check Balanced String
// https://leetcode.com/problems/check-balanced-string/
// https://leetcode.cn/problems/check-balanced-string/
//
// Easy
//
// You are given a string num consisting of only digits. A string of digits is called balanced
// if the sum of the digits at even indices is equal to the sum of digits at odd indices.
//
// Return true if num is balanced, otherwise return false.
//
// Example 1:
//
// Input: num = "1234"
//
// Output: false
//
// Explanation:
//
//     The sum of digits at even indices is 1 + 3 == 4, and the sum of digits at odd indices is 2 + 4 == 6.
//     Since 4 is not equal to 6, num is not balanced.
//
// Example 2:
//
// Input: num = "24123"
//
// Output: true
//
// Explanation:
//
//     The sum of digits at even indices is 2 + 1 + 3 == 6, and the sum of digits at odd indices is 4 + 2 == 6.
//     Since both are equal the num is balanced.
//
// Constraints:
//
//     2 <= num.length <= 100
//     num consists of digits only
//

struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (mut even_sum, mut odd_sum) = (0, 0);
        for (i, c) in num.chars().enumerate() {
            if i % 2 == 0 {
                even_sum += c.to_digit(10).unwrap();
            } else {
                odd_sum += c.to_digit(10).unwrap();
            }
        }
        even_sum == odd_sum
    }
}

#[test]
fn test() {
    assert!(!Solution::is_balanced("1234".to_string()));
    assert!(Solution::is_balanced("24123".to_string()));
    assert!(!Solution::is_balanced("123456".to_string()));
}
