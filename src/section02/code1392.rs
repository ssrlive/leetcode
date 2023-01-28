#![allow(dead_code)]

// 1392. Longest Happy Prefix
// https://leetcode.com/problems/longest-happy-prefix/
// https://leetcode.cn/problems/longest-happy-prefix/
//
// Hard
//
// A string is called a happy prefix if is a non-empty prefix which is also a suffix (excluding itself).
//
// Given a string s, return the longest happy prefix of s. Return an empty string "" if no such prefix exists.
//
// Example 1:
//
// Input: s = "level"
// Output: "l"
// Explanation: s contains 4 prefix excluding itself ("l", "le", "lev", "leve"), and suffix ("l", "el", "vel", "evel"). The largest prefix which is also suffix is given by "l".
//
// Example 2:
//
// Input: s = "ababab"
// Output: "abab"
// Explanation: "abab" is the largest prefix which is also suffix. They can overlap in the original string.
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    s contains only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s = s.as_bytes();
        let mut prefix = vec![0; s.len()];
        let mut j = 0;
        for i in 1..s.len() {
            while j > 0 && s[i] != s[j] {
                j = prefix[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            prefix[i] = j;
        }
        String::from_utf8(s[..prefix[s.len() - 1]].to_vec()).unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![("level", "l"), ("ababab", "abab"), ("leetcodeleet", "leet"), ("a", "")];
    for (s, expect) in cases {
        assert_eq!(Solution::longest_prefix(s.to_string()), expect);
    }
}
