#![allow(dead_code)]

// 2710. Remove Trailing Zeros From a String
// https://leetcode.com/problems/remove-trailing-zeros-from-a-string/
// https://leetcode.cn/problems/remove-trailing-zeros-from-a-string/
//
// Easy
//
// Given a positive integer num represented as a string, return the integer num without trailing zeros as a string.
//
// Example 1:
//
// Input: num = "51230100"
// Output: "512301"
// Explanation: Integer "51230100" has 2 trailing zeros, we remove them and return integer "512301".
//
// Example 2:
//
// Input: num = "123"
// Output: "123"
// Explanation: Integer "123" has no trailing zeros, we return integer "123".
//
// Constraints:
//
//     1 <= num.length <= 1000
//     num consists of only digits.
//     num doesn't have any leading zeros.
//

struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        num.trim_end_matches('0').to_string()
    }
}

#[test]
fn test() {
    let num = "51230100".to_string();
    let expect = "512301".to_string();
    let actual = Solution::remove_trailing_zeros(num);
    assert_eq!(expect, actual);

    let num = "123".to_string();
    let expect = "123".to_string();
    let actual = Solution::remove_trailing_zeros(num);
    assert_eq!(expect, actual);
}
