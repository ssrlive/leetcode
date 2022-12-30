#![allow(dead_code)]

// 890. Find and Replace Pattern
// https://leetcode.com/problems/find-and-replace-pattern/
// https://leetcode.cn/problems/find-and-replace-pattern/
//
// Given a list of strings words and a string pattern, return a list of words[i] that match pattern. You may return the answer in any order.
//
// A word matches the pattern if there exists a permutation of letters p so that after replacing every letter x in the pattern with p(x), we get the desired word.
//
// Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.
//
// Example 1:
//
// Input: words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
// Output: ["mee","aqq"]
// Explanation: "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
// "ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
//
// Example 2:
//
// Input: words = ["a","b","c"], pattern = "a"
// Output: ["a","b","c"]
//
// Constraints:
//
// - 1 <= pattern.length <= 20
// - 1 <= words.length <= 50
// - words[i].length == pattern.length
// - pattern and words[i] are lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn get_map(word: &String) -> String {
            let mut map = word.clone();
            for i in 0..word.len() {
                let c = word.chars().nth(i).unwrap();

                if c.is_ascii_alphabetic() {
                    map = map.replace(c, &i.to_string())
                }
            }
            map
        }
        words
            .iter()
            .cloned()
            .filter(|word| get_map(word) == get_map(&pattern))
            .collect()
    }
}

#[test]
fn test() {
    let words = vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
    let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    let pattern = "abb".to_string();
    let result = vec!["mee", "aqq"];
    assert_eq!(Solution::find_and_replace_pattern(words, pattern), result);

    let words = vec!["a", "b", "c"];
    let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    let pattern = "a".to_string();
    let result = vec!["a", "b", "c"];
    assert_eq!(Solution::find_and_replace_pattern(words, pattern), result);
}
