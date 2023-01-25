#![allow(dead_code)]

// 1316. Distinct Echo Substrings
// https://leetcode.com/problems/distinct-echo-substrings/
// https://leetcode.cn/problems/distinct-echo-substrings/
//
// Hard
//
// Return the number of distinct non-empty substrings of text that can be written as the concatenation
// of some string with itself (i.e. it can be written as a + a where a is some string).
//
// Example 1:
//
// Input: text = "abcabcabc"
// Output: 3
// Explanation: The 3 substrings are "abcabc", "bcabca" and "cabcab".
//
// Example 2:
//
// Input: text = "leetcodeleetcode"
// Output: 2
// Explanation: The 2 substrings are "ee" and "leetcodeleetcode".
//
// Constraints:
//
// -    1 <= text.length <= 2000
// -    text has only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        fn check(s: &[u8]) -> bool {
            let mut j = s.len() / 2;
            for i in 0..(s.len() / 2) {
                if s[i] != s[j] {
                    return false;
                }
                j += 1;
            }
            true
        }

        let mut s = std::collections::HashSet::new();
        let text = text.as_bytes();
        for i in 0..text.len() {
            let mut ans = vec![];
            for j in (i..text.len()).step_by(2) {
                ans.push(text[j]);
                if j + 1 < text.len() {
                    ans.push(text[j + 1]);
                } else {
                    break;
                }
                if check(&ans) {
                    s.insert(ans.clone());
                }
            }
        }
        s.len() as i32
    }
}

#[test]
fn test() {
    let cases = vec![("abcabcabc".to_string(), 3), ("leetcodeleetcode".to_string(), 2)];
    for (text, expect) in cases {
        assert_eq!(Solution::distinct_echo_substrings(text), expect);
    }
}
