#![allow(dead_code)]

/*

// 1763. Longest Nice Substring
// https://leetcode.com/problems/longest-nice-substring/
// https://leetcode.cn/problems/longest-nice-substring/
//
// Easy
//
// A string s is nice if, for every letter of the alphabet that s contains, it appears both in uppercase and lowercase. For example, "abABB" is nice because 'A' and 'a' appear, and 'B' and 'b' appear. However, "abA" is not because 'b' appears, but 'B' does not.

Given a string s, return the longest substring of s that is nice. If there are multiple, return the substring of the earliest occurrence. If there are none, return an empty string.

Example 1:

Input: s = "YazaAay"
Output: "aAa"
Explanation: "aAa" is a nice string because 'A/a' is the only letter of the alphabet in s, and both 'A' and 'a' appear.
"aAa" is the longest nice substring.

Example 2:

Input: s = "Bb"
Output: "Bb"
Explanation: "Bb" is a nice string because both 'B' and 'b' appear. The whole string is a substring.

Example 3:

Input: s = "c"
Output: ""
Explanation: There are no nice substrings.

Constraints:

    1 <= s.length <= 100
    s consists of uppercase and lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let sb = s.as_bytes();
        let mut max_range: (usize, usize) = (0, 0);

        for i in 0..(s.len() - 1) {
            let (mut lower_mask, mut upper_mask) = (0u32, 0u32);

            for (j, &sb_j) in sb.iter().enumerate().take(s.len()).skip(i) {
                match sb_j >= b'a' {
                    true => lower_mask |= 1 << (sb_j - b'a'),
                    false => upper_mask |= 1 << (sb_j - b'A'),
                };

                if lower_mask == upper_mask && (j + 1 - i) > (max_range.1 - max_range.0) {
                    max_range = (i, j + 1);
                }
            }
        }

        String::from_utf8(sb[max_range.0..max_range.1].to_vec()).unwrap()
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("YazaAay", "aAa"),
        ("Bb", "Bb"),
        ("c", ""),
    ];
    for (s, ans) in cases {
        assert_eq!(Solution::longest_nice_substring(s.to_string()), ans.to_string());
    }
}
