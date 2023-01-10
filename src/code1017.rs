#![allow(dead_code)]

// 1017. Convert to Base -2
// https://leetcode.com/problems/convert-to-base-2/
// https://leetcode.cn/problems/convert-to-base-2/
//
// Given an integer n, return a binary string representing its representation in base -2.
//
// Note that the returned string should not have leading zeros unless the string is "0".
//
// Example 1:
//
// Input: n = 2
// Output: "110"
// Explantion: (-2)2 + (-2)1 = 2
//
// Example 2:
//
// Input: n = 3
// Output: "111"
// Explantion: (-2)2 + (-2)1 + (-2)0 = 3
//
// Example 3:
//
// Input: n = 4
// Output: "100"
// Explantion: (-2)2 = 4
//
// Constraints:
//
// - 0 <= n <= 109
//

struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n = n;
        if n == 0 {
            return 0.to_string();
        }
        let mut result = String::new();
        while n != 0 {
            let rem = n % -2;

            if n < 0 && rem != 0 {
                result.push_str((2 + rem).to_string().as_str());
                n = 1 + n / -2;
            } else {
                result.push_str(rem.to_string().as_str());
                n /= -2;
            }
        }
        result.chars().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::base_neg2(2), "110");
    assert_eq!(Solution::base_neg2(3), "111");
    assert_eq!(Solution::base_neg2(4), "100");
    assert_eq!(Solution::base_neg2(0), "0");
    assert_eq!(Solution::base_neg2(1), "1");
}
