#![allow(dead_code)]

/*

// 2062. Count Vowel Substrings of a String
// https://leetcode.com/problems/count-vowel-substrings-of-a-string/
// https://leetcode.cn/problems/count-vowel-substrings-of-a-string/
//
// Easy
//
// A substring is a contiguous (non-empty) sequence of characters within a string.

A vowel substring is a substring that only consists of vowels ('a', 'e', 'i', 'o', and 'u') and has all five vowels present in it.

Given a string word, return the number of vowel substrings in word.

Example 1:

Input: word = "aeiouu"
Output: 2
Explanation: The vowel substrings of word are as follows (underlined):
- "aeiouu"
- "aeiouu"

Example 2:

Input: word = "unicornarihan"
Output: 0
Explanation: Not all 5 vowels are present, so there are no vowel substrings.

Example 3:

Input: word = "cuaieuouac"
Output: 7
Explanation: The vowel substrings of word are as follows (underlined):
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"

Constraints:

    1 <= word.length <= 100
    word consists of lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        use std::collections::HashSet;
        if word.len() < 5 {
            return 0;
        }
        let wb = word.as_bytes();
        let mut acc = HashSet::with_capacity(5);
        let vowels: HashSet<u8> = "aeiou".chars().map(|c| c as u8).collect();
        let mut res = 0;
        for i in 0..word.len() - 4 {
            for &b in wb[i..].iter().take_while(|b| vowels.contains(*b)) {
                acc.insert(b);
                if acc.len() == 5 {
                    res += 1;
                }
            }
            acc.clear();
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
    assert_eq!(Solution::count_vowel_substrings("unicornarihan".to_string()), 0);
    assert_eq!(Solution::count_vowel_substrings("cuaieuouac".to_string()), 7);
}
