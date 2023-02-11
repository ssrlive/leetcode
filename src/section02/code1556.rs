#![allow(dead_code)]

/*

// 1556. Thousand Separator
// https://leetcode.com/problems/thousand-separator/
// https://leetcode.cn/problems/thousand-separator/
//
// Easy
//
// Given an integer n, add a dot (".") as the thousands separator and return it in string format.

Example 1:

Input: n = 987
Output: "987"

Example 2:

Input: n = 1234
Output: "1.234"

Constraints:

    0 <= n <= 2^31 - 1
*/

struct Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut result = String::new();
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            if count == 3 {
                result.push('.');
                count = 0;
            }
            result.push((n % 10 + '0' as i32) as u8 as char);
            n /= 10;
            count += 1;
        }
        if result.is_empty() {
            return "0".to_string();
        }
        result.chars().rev().collect()
    }
}

#[test]
fn test() {
    let cases = vec![(987, "987"), (1234, "1.234"), (0, "0")];
    for (n, expected) in cases {
        assert_eq!(Solution::thousand_separator(n), expected);
    }
}
