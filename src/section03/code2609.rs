#![allow(dead_code)]

/*

// 2609. Find the Longest Balanced Substring of a Binary String
// https://leetcode.com/problems/find-the-longest-balanced-substring-of-a-binary-string/
// https://leetcode.cn/problems/find-the-longest-balanced-substring-of-a-binary-string/
//
// Easy
//
// You are given a binary string s consisting only of zeroes and ones.

A substring of s is considered balanced if all zeroes are before ones and the number of zeroes is equal to the number of ones inside the substring. Notice that the empty substring is considered a balanced substring.

Return the length of the longest balanced substring of s.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "01000111"
Output: 6
Explanation: The longest balanced substring is "000111", which has length 6.

Example 2:

Input: s = "00111"
Output: 4
Explanation: The longest balanced substring is "0011", which has length 4.

Example 3:

Input: s = "111"
Output: 0
Explanation: There is no balanced substring except the empty substring, so the answer is 0.

Constraints:

    1 <= s.length <= 50
    '0' <= s[i] <= '1'
*/

struct Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut res = 0;
        let mut temp = "01".to_string();
        while temp.len() <= s.len() {
            if s.contains(&temp) {
                res = temp.len() as i32;
            }
            temp = format!("0{}1", temp);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![("01000111", 6), ("00111", 4), ("111", 0), ("01110011", 4), ("0111001101110011", 4)];
    for (s, expect) in cases {
        assert_eq!(Solution::find_the_longest_balanced_substring(s.to_string()), expect);
    }
}
