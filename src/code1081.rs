#![allow(dead_code)]

// 1081. Smallest Subsequence of Distinct Characters
// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
// https://leetcode.cn/problems/smallest-subsequence-of-distinct-characters/
//
// Given a string s, return the lexicographically smallest subsequence of s that contains all the distinct characters of s exactly once.
//
// Example 1:
//
// Input: s = "bcabc"
// Output: "abc"
//
// Example 2:
//
// Input: s = "cbacdcbc"
// Output: "acdb"
//
// Constraints:
//
// - 1 <= s.length <= 1000
// - s consists of lowercase English letters.
//
// Note: This question is the same as 316: https://leetcode.com/problems/remove-duplicate-letters/
//

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        use std::collections::VecDeque;
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        let mut flag = vec![0; 26];
        let mut sk = VecDeque::<char>::new();

        for c in s.chars() {
            count[(c as u8 - b'a') as usize] -= 1;
            if flag[(c as u8 - b'a') as usize] == 1 {
                continue;
            }

            while let Some(ch) = sk.back() {
                let cc = *ch;
                if count[(cc as u8 - b'a') as usize] == 0 {
                    break;
                }
                if cc < c {
                    break;
                }
                sk.pop_back();
                flag[(cc as u8 - b'a') as usize] = 0;
            }

            sk.push_back(c);
            flag[(c as u8 - b'a') as usize] = 1;
        }

        sk.into_iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![("bcabc", "abc"), ("cbacdcbc", "acdb"), ("cdadabcc", "adbc")];
    for (s, expect) in cases {
        assert_eq!(Solution::smallest_subsequence(s.to_string()), expect);
    }
}
