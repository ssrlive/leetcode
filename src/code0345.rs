#![allow(dead_code)]

// 345. Reverse Vowels of a String
// https://leetcode.com/problems/reverse-vowels-of-a-string/
// https://leetcode.cn/problems/reverse-vowels-of-a-string/
//
// Given a string s, reverse only all the vowels in the string and return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
//
// Example 1:
//
// Input: s = "hello"
// Output: "holle"
//
// Example 2:
//
// Input: s = "leetcode"
// Output: "leotcede"
//
// Constraints:
//
// - 1 <= s.length <= 3 * 10^5
// - s consist of printable ASCII characters.
//

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !Self::is_vowel(chars[i]) {
                i += 1;
                continue;
            }
            if !Self::is_vowel(chars[j]) {
                j -= 1;
                continue;
            }
            chars.swap(i, j);
            i += 1;
            j -= 1;
        }
        chars.into_iter().collect()
    }

    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
}
