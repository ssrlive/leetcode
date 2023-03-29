#![allow(dead_code)]

/*

// 2207. Maximize Number of Subsequences in a String
// https://leetcode.com/problems/maximize-number-of-subsequences-in-a-string/
// https://leetcode.cn/problems/maximize-number-of-subsequences-in-a-string/
//
// Medium
//
// You are given a 0-indexed string text and another 0-indexed string pattern of length 2, both of which consist of only lowercase English letters.

You can add either pattern[0] or pattern[1] anywhere in text exactly once. Note that the character can be added even at the beginning or at the end of text.

Return the maximum number of times pattern can occur as a subsequence of the modified text.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

Example 1:

Input: text = "abdcdbc", pattern = "ac"
Output: 4
Explanation:
If we add pattern[0] = 'a' in between text[1] and text[2], we get "abadcdbc". Now, the number of times "ac" occurs as a subsequence is 4.
Some other strings which have 4 subsequences "ac" after adding a character to text are "aabdcdbc" and "abdacdbc".
However, strings such as "abdcadbc", "abdccdbc", and "abdcdbcc", although obtainable, have only 3 subsequences "ac" and are thus suboptimal.
It can be shown that it is not possible to get more than 4 subsequences "ac" by adding only one character.

Example 2:

Input: text = "aabb", pattern = "ab"
Output: 6
Explanation:
Some of the strings which can be obtained from text and have 6 subsequences "ab" are "aaabb", "aaabb", and "aabbb".

Constraints:

    1 <= text.length <= 10^5
    pattern.length == 2
    text and pattern consist only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let n = text.len();
        let s = text.chars().collect::<Vec<char>>();
        let p = pattern.chars().collect::<Vec<char>>();
        let (p1, p2) = (p[0], p[1]);
        if p1 == p2 {
            let mut count = 0;
            for c in s {
                if c == p1 {
                    count += 1;
                }
            }
            return count * (count + 1) / 2;
        }
        let mut result = 0;
        let mut count = 0;
        for &c in &s {
            if c == p1 {
                count += 1;
            } else if c == p2 {
                result += count;
            }
        }
        let mut memo1 = 0;
        let mut memo2 = 0;
        for &s_i in s.iter() {
            if s_i == p1 {
                memo1 += 1;
            }
        }
        for i in (0..n).rev() {
            if s[i] == p2 {
                memo2 += 1;
            }
        }
        (memo1 + result).max(memo2 + result)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("abdcdbc", "ac", 4),
        ("aabb", "ab", 6),
        ("abab", "ab", 5),
    ];
    for (t, p, e) in cases {
        assert_eq!(Solution::maximum_subsequence_count(t.to_string(), p.to_string()), e);
    }
}
