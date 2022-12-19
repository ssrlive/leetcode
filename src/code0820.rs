#![allow(dead_code)]

// 820. Short Encoding of Words
// https://leetcode.com/problems/short-encoding-of-words/
// https://leetcode.cn/problems/short-encoding-of-words/
//
// A valid encoding of an array of words is any reference string s and array of indices indices such that:
//
// words.length == indices.length
// The reference string s ends with the '#' character.
// For each index indices[i], the substring of s starting from indices[i] and up to (but not including) the next '#' character is equal to words[i].
// Given an array of words, return the length of the shortest reference string s possible of any valid encoding of words.
//
// Example 1:
//
// Input: words = ["time", "me", "bell"]
// Output: 10
// Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
// words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#' is underlined in "time#bell#"
// words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#' is underlined in "time#bell#"
// words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#' is underlined in "time#bell#"
//
// Example 2:
//
// Input: words = ["t"]
// Output: 2
// Explanation: A valid encoding would be s = "t#" and indices = [0].
//
// Constraints:
//
// - 1 <= words.length <= 2000
// - 1 <= words[i].length <= 7
// - words[i] consists of only lowercase letters.
//

struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut set: HashSet<String> = HashSet::new();
        for word in words {
            set.insert(word);
        }
        let v = set.iter().cloned().collect::<Vec<_>>();
        for word in v.iter() {
            for i in 1..word.len() {
                set.remove(&word[i..]);
            }
        }
        set.iter().fold(0, |acc, word| acc + word.len() as i32 + 1)
    }
}

#[test]
fn test() {
    let words = vec!["time", "me", "bell"];
    let words = words.iter().map(|word| word.to_string()).collect::<Vec<_>>();
    let result = 10;
    assert_eq!(Solution::minimum_length_encoding(words), result);

    let words = vec!["t".to_string()];
    let result = 2;
    assert_eq!(Solution::minimum_length_encoding(words), result);
}
