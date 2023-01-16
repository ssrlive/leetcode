#![allow(dead_code)]

// 854. K-Similar Strings
// https://leetcode.com/problems/k-similar-strings/
// https://leetcode.cn/problems/k-similar-strings/
//
// Strings s1 and s2 are k-similar (for some non-negative integer k) if we can swap the positions of two letters
// in s1 exactly k times so that the resulting string equals s2.
//
// Given two anagrams s1 and s2, return the smallest k for which s1 and s2 are k-similar.
//
// Example 1:
//
// Input: s1 = "ab", s2 = "ba"
// Output: 1
// Explanation: The two string are 1-similar because we can use one swap to change s1 to s2: "ab" --> "ba".
//
// Example 2:
//
// Input: s1 = "abc", s2 = "bca"
// Output: 2
// Explanation: The two strings are 2-similar because we can use two swaps to change s1 to s2: "abc" --> "bac" --> "bca".
//
// Constraints:
//
// - 1 <= s1.length <= 20
// - s2.length == s1.length
// - s1 and s2 contain only lowercase letters from the set {'a', 'b', 'c', 'd', 'e', 'f'}.
// - s2 is an anagram of s1.
//

struct Solution;

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        fn _k_similarity(s1: String, s2: String) -> Option<i32> {
            let s1 = s1.as_bytes();
            let s2 = s2.as_bytes();
            let n = s1.len();
            let mut result = n as i32 - 1;
            for i in 0..n {
                if s1[i] == s2[i] {
                    continue;
                }
                let mut matches = Vec::new();
                for j in i + 1..n {
                    if s1[j] == s2[j] || s1[j] != s2[i] {
                        continue;
                    }
                    matches.push(j);
                    if s1[i] != s2[j] {
                        continue;
                    }
                    let mut s1 = s1.to_vec();
                    s1.swap(i, j);
                    let v = 1 + _k_similarity(String::from_utf8(s1).ok()?, String::from_utf8(s2.to_vec()).ok()?)?;
                    return Some(v);
                }
                for j in matches {
                    let mut s1 = s1.to_vec();
                    s1.swap(i, j);
                    result = result
                        .min(1 + _k_similarity(String::from_utf8(s1).ok()?, String::from_utf8(s2.to_vec()).ok()?)?);
                }
                return Some(result);
            }
            Some(0)
        }
        _k_similarity(s1, s2).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::k_similarity("ab".to_string(), "ba".to_string()), 1);
    assert_eq!(Solution::k_similarity("abc".to_string(), "bca".to_string()), 2);
    assert_eq!(Solution::k_similarity("abac".to_string(), "baca".to_string()), 2);
    assert_eq!(Solution::k_similarity("aabc".to_string(), "abca".to_string()), 2);
    let (s1, s2) = ("abccaacceecdeea".to_string(), "bcaacceeccdeaae".to_string());
    assert_eq!(Solution::k_similarity(s1, s2), 9);
}
