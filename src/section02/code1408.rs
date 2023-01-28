#![allow(dead_code)]

// 1408. String Matching in an Array
// https://leetcode.com/problems/string-matching-in-an-array/
// https://leetcode.cn/problems/string-matching-in-an-array/
//
// Easy
//
// Given an array of string words, return all strings in words that is a substring of another word.
// You can return the answer in any order.
//
// A substring is a contiguous sequence of characters within a string
//
// Example 1:
//
// Input: words = ["mass","as","hero","superhero"]
// Output: ["as","hero"]
// Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
// ["hero","as"] is also a valid answer.
//
// Example 2:
//
// Input: words = ["leetcode","et","code"]
// Output: ["et","code"]
// Explanation: "et", "code" are substring of "leetcode".
//
// Example 3:
//
// Input: words = ["blue","green","bu"]
// Output: []
// Explanation: No string of words is substring of another string.
//
// Constraints:
//
// -    1 <= words.length <= 100
// -    1 <= words[i].length <= 30
// -    words[i] contains only lowercase English letters.
// -    All the strings of words are unique.
//

struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let s = words.join("_");
        let f = |word: &String| s.matches(word).nth(1).is_some();
        words.into_iter().filter(f).collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["mass", "as", "hero", "superhero"], vec!["as", "hero"]),
        (vec!["leetcode", "et", "code"], vec!["et", "code"]),
        (vec!["blue", "green", "bu"], vec![]),
    ];
    for (words, expect) in cases {
        let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        let expect: Vec<String> = expect.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::string_matching(words), expect);
    }
}
