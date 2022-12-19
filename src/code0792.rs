#![allow(dead_code)]

// 792. Number of Matching Subsequences
// https://leetcode.com/problems/number-of-matching-subsequences/
// https://leetcode.cn/problems/number-of-matching-subsequences/
//
// Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.
//
// A subsequence of a string is a new string generated from the original string with some characters (can be none)
// deleted without changing the relative order of the remaining characters.
//
// For example, "ace" is a subsequence of "abcde".
//
// Example 1:
//
// Input: s = "abcde", words = ["a","bb","acd","ace"]
// Output: 3
// Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
//
// Example 2:
//
// Input: s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
// Output: 2
//
// Constraints:
//
// - 1 <= s.length <= 5 * 10^4
// - 1 <= words.length <= 5000
// - 1 <= words[i].length <= 50
// - s and words[i] consist of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut waiting = vec![Vec::new(); 26];
        for w in &words {
            let w = w.as_bytes();
            waiting[(w[0] - b'a') as usize].push(&w[1..]);
        }
        let mut answer = 0;
        for &u in s.as_bytes() {
            for w in &waiting[(u - b'a') as usize].drain(..).collect::<Vec<_>>() {
                if w.is_empty() {
                    answer += 1;
                } else {
                    waiting[(w[0] - b'a') as usize].push(&w[1..]);
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    let words = vec!["a", "bb", "acd", "ace"];
    let words = words.iter().map(|&s| s.to_owned()).collect();
    assert_eq!(Solution::num_matching_subseq("abcde".to_owned(), words), 3);

    let words = vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
    let words = words.iter().map(|&s| s.to_owned()).collect();
    assert_eq!(Solution::num_matching_subseq("dsahjpjauf".to_owned(), words), 2);
}
