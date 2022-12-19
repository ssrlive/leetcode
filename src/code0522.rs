#![allow(dead_code)]

// 522. Longest Uncommon Subsequence II
// https://leetcode.com/problems/longest-uncommon-subsequence-ii/
// https://leetcode.cn/problems/longest-uncommon-subsequence-ii/
//
// Given an array of strings strs, return the length of the longest uncommon subsequence between them. If the longest uncommon subsequence does not exist, return -1.
//
// An uncommon subsequence between an array of strings is a string that is a subsequence of one string but not the others.
//
// A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.
//
// For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "aebdc" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).
//
// Example 1:
//
// Input: strs = ["aba","cdc","eae"]
// Output: 3
//
// Example 2:
//
// Input: strs = ["aaa","aaa","aa"]
// Output: -1
//
// Constraints:
//
// - 2 <= strs.length <= 50
// - 1 <= strs[i].length <= 10
// - strs[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        fn is_subsequence(s1: &str, s2: &str) -> Option<bool> {
            let mut i = 0;
            for c in s2.chars() {
                if i == s1.len() {
                    break;
                }
                if c == s1.chars().nth(i)? {
                    i += 1;
                }
            }
            Some(i == s1.len())
        }

        let mut strs = strs;
        strs.sort_by_key(|s| s.len());
        strs.reverse();

        for i in 0..strs.len() {
            let mut flag = false;
            for j in 0..strs.len() {
                if i == j {
                    continue;
                }
                if is_subsequence(&strs[i], &strs[j]).unwrap_or(false) {
                    flag = true;
                    break;
                }
            }
            if !flag {
                return strs[i].len() as i32;
            }
        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_lu_slength(vec!["aba".to_string(), "cdc".to_string(), "eae".to_string()]),
        3
    );
    assert_eq!(
        Solution::find_lu_slength(vec!["aaa".to_string(), "aaa".to_string(), "aa".to_string()]),
        -1
    );
}
