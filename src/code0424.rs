#![allow(dead_code)]

// 424. Longest Repeating Character Replacement
// https://leetcode.com/problems/longest-repeating-character-replacement/
//
// You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.
//
// Return the length of the longest substring containing the same letter you can get after performing the above operations.
//
// Example 1:
//
// Input: s = "ABAB", k = 2
// Output: 4
// Explanation: Replace the two 'A's with two 'B's or vice versa.
//
// Example 2:
//
// Input: s = "AABABBA", k = 1
// Output: 4
// Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
// The substring "BBBB" has the longest repeating letters, which is 4.
//
// Constraints:
//
// - 1 <= s.length <= 105
// - s consists of only uppercase English letters.
// - 0 <= k <= s.length
//

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        fn _character_replacement(s: String, k: i32) -> Option<i32> {
            let mut map = std::collections::HashMap::new();
            let mut max_count = 0;
            let mut result = 0;
            let mut left = 0;
            for (right, c) in s.chars().enumerate() {
                *map.entry(c).or_insert(0) += 1;
                max_count = std::cmp::max(max_count, *map.get(&c)?);
                while right - left + 1 - max_count > k as usize {
                    *map.get_mut(&s.chars().nth(left)?)? -= 1;
                    left += 1;
                }
                result = std::cmp::max(result, right - left + 1);
            }
            Some(result as i32)
        }

        _character_replacement(s, k).unwrap_or(0)
    }
}

#[test]
fn test_character_replacement() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
