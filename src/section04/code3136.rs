#![allow(dead_code)]

// 3136. Valid Word
// https://leetcode.com/problems/valid-word/
// https://leetcode.cn/problems/valid-word/
//
// Easy
//
// A word is considered valid if:
//
// - It contains a minimum of 3 characters.
// - It consists of the digits 0-9, and the uppercase and lowercase English letters. (Not necessary to have all of them.)
// - It includes at least one vowel.
// - It includes at least one consonant.
//
// You are given a string word.
//
// Return true if word is valid, otherwise, return false.
//
// Notes:
//
// - 'a', 'e', 'i', 'o', 'u', and their uppercases are vowels.
// - A consonant is an English letter that is not a vowel.
//
// Example 1:
//
// Input: word = "234Adas"
//
// Output: true
//
// Explanation:
//
// This word satisfies the conditions.
//
// Example 2:
//
// Input: word = "b3"
//
// Output: false
//
// Explanation:
//
// The length of this word is fewer than 3, and does not have a vowel.
//
// Example 3:
//
// Input: word = "a3$e"
//
// Output: false
//
// Explanation:
//
// This word contains a '$' character and does not have a consonant.
//
// Constraints:
//
// 1 <= word.length <= 20
// word consists of English uppercase and lowercase letters, digits, '@', '#', and '$'.
//

struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let n = word.len();
        if n < 3 {
            return false;
        }
        let word = word.to_lowercase();
        let v = word.matches(|c: char| "aeiou".contains(c)).count();
        let c = word.chars().filter(|c| c.is_alphabetic() && !"aeiou".contains(*c)).count();
        let non_alphanumeric = word.chars().any(|c| !c.is_alphabetic() && !c.is_ascii_digit());
        v >= 1 && c >= 1 && !non_alphanumeric
    }
}

#[test]
fn test() {
    assert!(Solution::is_valid("234Adas".to_string()));
    assert!(!Solution::is_valid("b3".to_string()));
    assert!(!Solution::is_valid("a3$e".to_string()));
}
