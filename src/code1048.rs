#![allow(dead_code)]

// 1048. Longest String Chain
// https://leetcode.com/problems/longest-string-chain/
// https://leetcode.cn/problems/longest-string-chain/
//
// You are given an array of words where each word consists of lowercase English letters.
//
// wordA is a predecessor of wordB if and only if we can insert exactly one letter anywhere in
// wordA without changing the order of the other characters to make it equal to wordB.
//
// For example, "abc" is a predecessor of "abac", while "cba" is not a predecessor of "bcad".
// A word chain is a sequence of words [word1, word2, ..., wordk] with k >= 1, where word1 is a predecessor of
// word2, word2 is a predecessor of word3, and so on. A single word is trivially a word chain with k == 1.
//
// Return the length of the longest possible word chain with words chosen from the given list of words.
//
// Example 1:
//
// Input: words = ["a","b","ba","bca","bda","bdca"]
// Output: 4
// Explanation: One of the longest word chains is ["a","ba","bda","bdca"].
//
// Example 2:
//
// Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
// Output: 5
// Explanation: All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
//
// Example 3:
//
// Input: words = ["abcd","dbqca"]
// Output: 1
// Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
// ["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
//
// Constraints:
//
// - 1 <= words.length <= 1000
// - 1 <= words[i].length <= 16
// - words[i] only consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut words = words;
        words.sort_by_cached_key(String::len);
        let mut hm = HashMap::new();
        let mut answer = 0;
        for word in &words {
            let max = (0..word.len())
                .filter_map(|i| hm.get(&(String::new() + &word[0..i] + &word[i + 1..])))
                .max()
                .unwrap_or(&0)
                + 1;
            hm.insert(word, max);
            answer = answer.max(max);
        }
        answer
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["a", "b", "ba", "bca", "bda", "bdca"], 4),
        (vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"], 5),
        (vec!["abcd", "dbqca"], 1),
    ];
    for (words, expected) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(Solution::longest_str_chain(words), expected);
    }
}
