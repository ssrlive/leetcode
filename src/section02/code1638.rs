#![allow(dead_code)]

/*

// 1638. Count Substrings That Differ by One Character
// https://leetcode.com/problems/count-substrings-that-differ-by-one-character/
// https://leetcode.cn/problems/count-substrings-that-differ-by-one-character/
//
// Medium
//
// Given two strings s and t, find the number of ways you can choose a non-empty substring of s and replace a single character by a different character such that the resulting substring is a substring of t. In other words, find the number of substrings in s that differ from some substring in t by exactly one character.

For example, the underlined substrings in "computer" and "computation" only differ by the 'e'/'a', so this is a valid way.

Return the number of substrings that satisfy the condition above.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "aba", t = "baba"
Output: 6
Explanation: The following are the pairs of substrings from s and t that differ by exactly 1 character:
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
The underlined portions are the substrings that are chosen from s and t.

​​Example 2:

Input: s = "ab", t = "bb"
Output: 3
Explanation: The following are the pairs of substrings from s and t that differ by 1 character:
("ab", "bb")
("ab", "bb")
("ab", "bb")
​​​​The underlined portions are the substrings that are chosen from s and t.

Constraints:

    1 <= s.length, t.length <= 100
    s and t consist of lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let (n, m) = (s.len(), t.len());
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let mut dp1 = vec![vec![0; m + 1]; n + 1];
        let mut ans = 0;

        for i in 1..=n {
            for j in 1..=m {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                    dp1[i][j] = dp1[i - 1][j - 1];
                } else {
                    dp1[i][j] = 1 + dp[i - 1][j - 1];
                }
                ans += dp1[i][j];
            }
        }

        ans
    }
}

#[test]
fn test() {
    let cases = vec![("aba", "baba", 6), ("ab", "bb", 3), ("a", "a", 0), ("abe", "bbc", 10)];
    for (s, t, expect) in cases {
        assert_eq!(Solution::count_substrings(s.to_string(), t.to_string()), expect);
    }
}
