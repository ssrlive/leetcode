#![allow(dead_code)]

// 67. Add Binary
// https://leetcode.com/problems/add-binary/
// https://leetcode.cn/problems/add-binary/
//
// Given two binary strings a and b, return their sum as a binary string.
//
// Example 1:
//
// Input: a = "11", b = "1"
// Output: "100"
//
// Example 2:
//
// Input: a = "1010", b = "1011"
// Output: "10101"
//
// Constraints:
//
// - 1 <= a.length, b.length <= 104
// - a and b consist only of '0' or '1' characters.
// - Each string does not contain leading zeros except for the zero itself.
//

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            u128::from_str_radix(a.as_str(), 2).unwrap() + u128::from_str_radix(b.as_str(), 2).unwrap()
        )
    }
}

#[test]
fn test() {
    let cased = vec![
        ("11", "1", "100"),
        ("1010", "1011", "10101"),
        ("0", "0", "0"),
        ("1", "0", "1"),
        ("0", "1", "1"),
        ("1", "1", "10"),
        ("10", "1", "11"),
        ("1", "10", "11"),
        ("10", "10", "100"),
        ("11", "11", "110"),
        ("101", "11", "1000"),
        ("11", "101", "1000"),
        ("101", "101", "1010"),
        ("1010", "101", "1111"),
        ("101", "1010", "1111"),
        ("1010", "1010", "10100"),
    ];
    for (a, b, expected) in cased {
        assert_eq!(Solution::add_binary(a.to_string(), b.to_string()), expected.to_string());
    }
}
