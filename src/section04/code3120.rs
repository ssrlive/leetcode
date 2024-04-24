#![allow(dead_code)]

// 3120. Count the Number of Special Characters I
// https://leetcode.com/problems/count-the-number-of-special-characters-i/
// https://leetcode.cn/problems/count-the-number-of-special-characters-i/
//
// Easy
//
// You are given a string word. A letter is called special if it appears both in lowercase and uppercase in word.
//
// Return the number of special letters in word.
//
// Example 1:
//
// Input: word = "aaAbcBC"
//
// Output: 3
//
// Explanation:
//
// The special characters in word are 'a', 'b', and 'c'.
//
// Example 2:
//
// Input: word = "abc"
//
// Output: 0
//
// Explanation:
//
// No character in word appears in uppercase.
//
// Example 3:
//
// Input: word = "abBCab"
//
// Output: 1
//
// Explanation:
//
// The only special character in word is 'b'.
//
// Constraints:
//
// 1 <= word.length <= 50
// word consists of only lowercase and uppercase English letters.
//

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        word.chars()
            .fold([0; 26], |mut map, c| {
                let i = c.to_ascii_lowercase() as usize - 'a' as usize;
                if c.is_uppercase() {
                    map[i] |= 0b10;
                } else {
                    map[i] |= 0b01;
                }
                map
            })
            .iter()
            .filter(|x| x.eq(&&0b11))
            .count() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
    assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
    assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
}
