#![allow(dead_code)]

// 131. Palindrome Partitioning
// https://leetcode.com/problems/palindrome-partitioning/
//
// Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
//
// A palindrome string is a string that reads the same backward as forward.

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &str) -> bool {
            let mut i = 0;
            let mut j = s.len() - 1;
            while i < j {
                if s.as_bytes()[i] != s.as_bytes()[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn backtrack(s: &str, start: usize, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
            if start == s.len() {
                result.push(path.clone());
                return;
            }
            for end in start..s.len() {
                if is_palindrome(&s[start..=end]) {
                    path.push(s[start..=end].to_string());
                    backtrack(s, end + 1, path, result);
                    path.pop();
                }
            }
        }

        let mut result = vec![];
        let mut path = vec![];
        backtrack(&s, 0, &mut path, &mut result);
        result
    }
}

#[test]
fn test_partition() {
    let s = "aab".to_string();
    let result = vec![vec!["a", "a", "b"], vec!["aa", "b"]];
    assert_eq!(Solution::partition(s), result);

    let s = "a".to_string();
    let result = vec![vec!["a"]];
    assert_eq!(Solution::partition(s), result);
}
