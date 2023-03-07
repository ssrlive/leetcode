#![allow(dead_code)]

/*

// 2311. Longest Binary Subsequence Less Than or Equal to K
// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/
// https://leetcode.cn/problems/longest-binary-subsequence-less-than-or-equal-to-k/
//
// Medium
//
// You are given a binary string s and a positive integer k.

Return the length of the longest subsequence of s that makes up a binary number less than or equal to k.

Note:

    The subsequence can contain leading zeroes.
    The empty string is considered to be equal to 0.
    A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

Example 1:

Input: s = "1001010", k = 5
Output: 5
Explanation: The longest subsequence of s that makes up a binary number less than or equal to 5 is "00010", as this number is equal to 2 in decimal.
Note that "00100" and "00101" are also possible, which are equal to 4 and 5 in decimal, respectively.
The length of this subsequence is 5, so 5 is returned.

Example 2:

Input: s = "00101001", k = 1
Output: 6
Explanation: "000001" is the longest subsequence of s that makes up a binary number less than or equal to 1, as this number is equal to 1 in decimal.
The length of this subsequence is 6, so 6 is returned.

Constraints:

    1 <= s.length <= 1000
    s[i] is either '0' or '1'.
    1 <= k <= 10^9
*/

struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut cur = 0;
        let mut exp = 0;
        for c in s.chars().rev() {
            if c == '1' && (exp >= 32 || 2i64.pow(exp) > k as i64 - cur) {
                continue;
            }
            cur += 2i64.pow(exp) * (c == '1') as i64;
            res += 1;
            exp += 1;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("1001010", 5, 5),
        ("00101001", 1, 6),
        ("1101010101010110111001011010010010100101", 5, 21),
    ];
    for (s, k, expect) in cases {
        assert_eq!(Solution::longest_subsequence(s.to_string(), k), expect);
    }
}
