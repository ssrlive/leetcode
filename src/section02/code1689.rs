#![allow(dead_code)]

/*

// 1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
// https://leetcode.cn/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
//
// Medium
//
// A decimal number is called deci-binary if each of its digits is either 0 or 1 without any leading zeros. For example, 101 and 1100 are deci-binary, while 112 and 3001 are not.

Given a string n that represents a positive decimal integer, return the minimum number of positive deci-binary numbers needed so that they sum up to n.

Example 1:

Input: n = "32"
Output: 3
Explanation: 10 + 11 + 11 = 32

Example 2:

Input: n = "82734"
Output: 8

Example 3:

Input: n = "27346209830709182346"
Output: 9

Constraints:

    1 <= n.length <= 10^5
    n consists of only digits.
    n does not contain any leading zeros and represents a positive integer.
*/

struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().map(|c| c.to_digit(10).unwrap() as i32).max().unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![("32", 3), ("82734", 8), ("27346209830709182346", 9)];
    for (n, expected) in cases {
        assert_eq!(Solution::min_partitions(n.to_string()), expected);
    }
}
