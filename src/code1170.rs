#![allow(dead_code)]

// 1170. Compare Strings by Frequency of the Smallest Character
// https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character/
// https://leetcode.cn/problems/compare-strings-by-frequency-of-the-smallest-character/
//
// Let the function f(s) be the frequency of the lexicographically smallest character in a non-empty string s.
// For example, if s = "dcce" then f(s) = 2 because the lexicographically smallest character is 'c', which has a frequency of 2.
//
// You are given an array of strings words and another array of query strings queries.
// For each query queries[i], count the number of words in words such that f(queries[i]) < f(W) for each W in words.
//
// Return an integer array answer, where each answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: queries = ["cbd"], words = ["zaaaz"]
// Output: [1]
// Explanation: On the first query we have f("cbd") = 1, f("zaaaz") = 3 so f("cbd") < f("zaaaz").
//
// Example 2:
//
// Input: queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
// Output: [1,2]
// Explanation: On the first query only f("bbb") < f("aaaa"). On the second query both f("aaa") and f("aaaa") are both > f("cc").
//
// Constraints:
//
// - 1 <= queries.length <= 2000
// - 1 <= words.length <= 2000
// - 1 <= queries[i].length, words[i].length <= 10
// - queries[i][j], words[i][j] consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        fn get_freq(word: String) -> usize {
            word.matches(word.chars().min().unwrap()).count()
        }

        let mut word_freq = [0; 12];
        words.into_iter().for_each(|word| word_freq[get_freq(word)] += 1);
        for i in (0..10).rev() {
            word_freq[i] += word_freq[i + 1];
        }
        queries.into_iter().map(|q| word_freq[get_freq(q) + 1]).collect()
    }
}

#[test]
fn test() {
    let queries = vec!["cbd".to_string()];
    let words = vec!["zaaaz".to_string()];
    let result = vec![1];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), result);

    let queries = vec!["bbb".to_string(), "cc".to_string()];
    let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()];
    let result = vec![1, 2];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), result);
}
