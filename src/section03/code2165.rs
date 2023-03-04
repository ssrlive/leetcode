#![allow(dead_code)]

/*

// 2165. Smallest Value of the Rearranged Number
// https://leetcode.com/problems/smallest-value-of-the-rearranged-number/
// https://leetcode.cn/problems/smallest-value-of-the-rearranged-number/
//
// Medium
//
// You are given an integer num. Rearrange the digits of num such that its value is minimized and it does not contain any leading zeros.

Return the rearranged number with minimal value.

Note that the sign of the number does not change after rearranging the digits.

Example 1:

Input: num = 310
Output: 103
Explanation: The possible arrangements for the digits of 310 are 013, 031, 103, 130, 301, 310.
The arrangement with the smallest value that does not contain any leading zeros is 103.

Example 2:

Input: num = -7605
Output: -7650
Explanation: Some possible arrangements for the digits of -7605 are -7650, -6705, -5076, -0567.
The arrangement with the smallest value that does not contain any leading zeros is -7650.

Constraints:

    -10^15 <= num <= 10^15
*/

struct Solution;

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let s = num.abs().to_string();
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort_by(|a, b| if num < 0 { b.cmp(a) } else { a.cmp(b) });
        if num > 0 {
            let i = s.iter().position(|&c| c != '0').unwrap();
            s.swap(0, i);
        }
        let s = s.iter().collect::<String>();
        s.parse::<i64>().unwrap() * (if num < 0 { -1 } else { 1 })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_number(310), 103);
    assert_eq!(Solution::smallest_number(-7605), -7650);
}
