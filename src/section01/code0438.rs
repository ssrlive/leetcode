#![allow(dead_code)]

// 438. Find All Anagrams in a String
// https://leetcode.com/problems/find-all-anagrams-in-a-string/
// https://leetcode.cn/problems/find-all-anagrams-in-a-string/
//
// Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
//
// Example 1:
//
// Input: s = "cbaebabacd", p = "abc"
// Output: [0,6]
// Explanation:
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".
//
// Example 2:
//
// Input: s = "abab", p = "ab"
// Output: [0,1,2]
// Explanation:
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".
//
// Constraints:
//
// 1 <= s.length, p.length <= 3 * 10^4
// s and p consist of lowercase English letters.
//
// Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
//

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut p_map = std::collections::HashMap::new();
        for c in p.chars() {
            *p_map.entry(c).or_insert(0) += 1;
        }
        let mut s_map = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            *s_map.entry(c).or_insert(0) += 1;
            if i >= p.len() {
                let c = s.chars().nth(i - p.len()).unwrap();
                if let Some(v) = s_map.get_mut(&c) {
                    *v -= 1;
                    if *v == 0 {
                        s_map.remove(&c);
                    }
                }
            }
            if s_map == p_map {
                answer.push(i as i32 - p.len() as i32 + 1);
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()), vec![0, 6]);
    assert_eq!(Solution::find_anagrams("abab".to_string(), "ab".to_string()), vec![0, 1, 2]);
}
