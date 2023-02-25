#![allow(dead_code)]

/*

// 1876. Substrings of Size Three with Distinct Characters
// https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/
// https://leetcode.cn/problems/substrings-of-size-three-with-distinct-characters/
//
// Easy
//
// A string is good if there are no repeated characters.

Given a string s​​​​​, return the number of good substrings of length three in s​​​​​​.

Note that if there are multiple occurrences of the same substring, every occurrence should be counted.

A substring is a contiguous sequence of characters in a string.

Example 1:

Input: s = "xyzzaz"
Output: 1
Explanation: There are 4 substrings of size 3: "xyz", "yzz", "zza", and "zaz".
The only good substring of length 3 is "xyz".

Example 2:

Input: s = "aababcabc"
Output: 4
Explanation: There are 7 substrings of size 3: "aab", "aba", "bab", "abc", "bca", "cab", and "abc".
The good substrings are "abc", "bca", "cab", and "abc".

Constraints:

    1 <= s.length <= 100
    s​​​​​​ consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let sb = s.as_bytes();
        sb.iter().enumerate().skip(2).fold(0, |res, (ind, b)| {
            res + (*b != sb[ind - 1] && *b != sb[ind - 2] && sb[ind - 1] != sb[ind - 2]) as i32
        })
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("xyzzaz", 1),
        ("aababcabc", 4)
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::count_good_substrings(s.to_string()), expect);
    }
}
