#![allow(dead_code)]

/*

// 1653. Minimum Deletions to Make String Balanced
// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
// https://leetcode.cn/problems/minimum-deletions-to-make-string-balanced/
//
// Medium
//
// You are given a string s consisting only of characters 'a' and 'b'​​​​.

You can delete any number of characters in s to make s balanced. s is balanced if there is no pair of indices (i,j) such that i < j and s[i] = 'b' and s[j]= 'a'.

Return the minimum number of deletions needed to make s balanced.

Example 1:

Input: s = "aababbab"
Output: 2
Explanation: You can either:
Delete the characters at 0-indexed positions 2 and 6 ("aababbab" -> "aaabbb"), or
Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").

Example 2:

Input: s = "bbaaaaabb"
Output: 2
Explanation: The only solution is to delete the first two characters.

Constraints:

    1 <= s.length <= 10^5
    s[i] is 'a' or 'b'​​.
*/

struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut memo = vec![(0, 0, 0, 0); n + 1];

        for i in 0..n {
            memo[i + 1].0 = memo[i].0;
            memo[i + 1].1 = memo[i].1;
            if s[i] == 'a' {
                memo[i + 1].0 += 1;
            } else {
                memo[i + 1].1 += 1;
            }
        }

        for i in (1..=n).rev() {
            memo[i - 1].2 = memo[i].2;
            memo[i - 1].3 = memo[i].3;
            if s[i - 1] == 'a' {
                memo[i - 1].2 += 1;
            } else {
                memo[i - 1].3 += 1;
            }
        }

        let mut result = n as i32;
        for item in memo.iter().take(n) {
            result = result.min(item.1 + item.3); // a
            result = result.min(item.0 + item.2); // b
            result = result.min(item.1 + item.2); // ab
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
}
