#![allow(dead_code)]

/*

// 2255. Count Prefixes of a Given String
// https://leetcode.com/problems/count-prefixes-of-a-given-string/
// https://leetcode.cn/problems/count-prefixes-of-a-given-string/
//
// Easy
//
// You are given a string array words and a string s, where words[i] and s comprise only of lowercase English letters.

Return the number of strings in words that are a prefix of s.

A prefix of a string is a substring that occurs at the beginning of the string. A substring is a contiguous sequence of characters within a string.

Example 1:

Input: words = ["a","b","c","ab","bc","abc"], s = "abc"
Output: 3
Explanation:
The strings in words which are a prefix of s = "abc" are:
"a", "ab", and "abc".
Thus the number of strings in words which are a prefix of s is 3.

Example 2:

Input: words = ["a","a"], s = "aa"
Output: 2
Explanation:
Both of the strings are a prefix of s.
Note that the same string can occur multiple times in words, and it should be counted each time.

Constraints:

    1 <= words.length <= 1000
    1 <= words[i].length, s.length <= 10
    words[i] and s consist of lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|w| s.starts_with(w.as_str())).count() as _
    }
}

#[test]
fn test() {
    let cases = vec![(vec!["a", "b", "c", "ab", "bc", "abc"], "abc", 3), (vec!["a", "a"], "aa", 2)];
    for (words, s, expected) in cases {
        let words = words.iter().map(|w| w.to_string()).collect();
        let s = s.to_string();
        assert_eq!(Solution::count_prefixes(words, s), expected);
    }
}
