#![allow(dead_code)]

/*

// 1961. Check If String Is a Prefix of Array
// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/
// https://leetcode.cn/problems/check-if-string-is-a-prefix-of-array/
//
// Easy
//
// Given a string s and an array of strings words, determine whether s is a prefix string of words.

A string s is a prefix string of words if s can be made by concatenating the first k strings in words for some positive k no larger than words.length.

Return true if s is a prefix string of words, or false otherwise.

Example 1:

Input: s = "iloveleetcode", words = ["i","love","leetcode","apples"]
Output: true
Explanation:
s can be made by concatenating "i", "love", and "leetcode" together.

Example 2:

Input: s = "iloveleetcode", words = ["apples","i","love","leetcode"]
Output: false
Explanation:
It is impossible to make s using a prefix of arr.

Constraints:

    1 <= words.length <= 100
    1 <= words[i].length <= 20
    1 <= s.length <= 1000
    words[i] and s consist of only lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut ls = s.len();
        let mut str_it = s.chars();
        for word in words.into_iter() {
            let lw = word.len();
            if ls < lw || !str_it.by_ref().take(lw).zip(word.chars()).all(|(c1, c2)| c1 == c2) {
                return false;
            } else if ls == lw {
                return true;
            }
            ls -= lw;
        }
        ls == 0
    }
}

#[test]
fn test() {
    let s = "iloveleetcode".to_string();
    let words = vec!["i".to_string(), "love".to_string(), "leetcode".to_string(), "apples".to_string()];
    assert!(Solution::is_prefix_string(s, words));

    let s = "iloveleetcode".to_string();
    let words = vec!["apples".to_string(), "i".to_string(), "love".to_string(), "leetcode".to_string()];
    assert!(!Solution::is_prefix_string(s, words));
}
