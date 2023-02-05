#![allow(dead_code)]

/*
// 1456. Maximum Number of Vowels in a Substring of Given Length
Medium

Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.

Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

Example 1:

Input: s = "abciiidef", k = 3
Output: 3
Explanation: The substring "iii" contains 3 vowel letters.

Example 2:

Input: s = "aeiou", k = 2
Output: 2
Explanation: Any substring of length 2 contains 2 vowels.

Example 3:

Input: s = "leetcode", k = 3
Output: 2
Explanation: "lee", "eet" and "ode" contain 2 vowels.

Constraints:

    1 <= s.length <= 10^5
    s consists of lowercase English letters.
    1 <= k <= s.length
*/

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(c: u8) -> bool {
            matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
        }

        let mut max = 0;
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;
        let k = k as usize;
        let s = s.as_bytes();
        while j < s.len() {
            if is_vowel(s[j]) {
                count += 1;
            }
            if j - i + 1 == k {
                max = max.max(count);
                if is_vowel(s[i]) {
                    count -= 1;
                }
                i += 1;
            }
            j += 1;
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
    assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
}
