#![allow(dead_code)]

// 1163. Last Substring in Lexicographical Order
// https://leetcode.com/problems/last-substring-in-lexicographical-order/
// https://leetcode.cn/problems/last-substring-in-lexicographical-order/
//
// Given a string s, return the last substring of s in lexicographical order.
//
// Example 1:
//
// Input: s = "abab"
// Output: "bab"
// Explanation: The substrings are ["a", "ab", "aba", "abab", "b", "ba", "bab"]. The lexicographically maximum substring is "bab".
//
// Example 2:
//
// Input: s = "leetcode"
// Output: "tcode"
//
// Constraints:
//
// - 1 <= s.length <= 4 * 10^5
// - s contains only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        let s = s.as_bytes();
        let n = s.len();
        while j + k < n {
            match s[i + k].cmp(&s[j + k]) {
                std::cmp::Ordering::Equal => k += 1,
                std::cmp::Ordering::Less => {
                    i = (i + k + 1).max(j);
                    j = i + 1;
                    k = 0;
                }
                std::cmp::Ordering::Greater => {
                    j += k + 1;
                    k = 0;
                }
            }
        }
        unsafe { String::from_utf8_unchecked(s[i..].to_vec()) }
    }
}

#[test]
fn test() {
    let cases = vec![("abab", "bab"), ("leetcode", "tcode")];
    for (s, expect) in cases {
        assert_eq!(Solution::last_substring(s.to_string()), expect);
    }
}
