#![allow(dead_code)]

// 151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string/
// https://leetcode.cn/problems/reverse-words-in-a-string/
//
// Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
// Note that s may contain leading or trailing spaces or multiple spaces between two words.
// The returned string should only have a single space separating the words. Do not include any extra spaces.
//

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = s.split_whitespace().collect::<Vec<&str>>();
        words.reverse();
        words.join(" ")
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reverse_words("the sky is blue".to_string()),
        "blue is sky the".to_string()
    );
    assert_eq!(Solution::reverse_words("  hello world  ".to_string()), "world hello".to_string());
    assert_eq!(
        Solution::reverse_words("a good   example".to_string()),
        "example good a".to_string()
    );
    assert_eq!(
        Solution::reverse_words("  Bob    Loves  Alice   ".to_string()),
        "Alice Loves Bob".to_string()
    );
    assert_eq!(
        Solution::reverse_words("Alice does not even like bob".to_string()),
        "bob like even not does Alice".to_string()
    );
}
