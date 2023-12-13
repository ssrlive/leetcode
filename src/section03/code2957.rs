#![allow(dead_code)]

// 2957. Remove Adjacent Almost-Equal Characters
// https://leetcode.com/problems/remove-adjacent-almost-equal-characters/
// https://leetcode.cn/problems/remove-adjacent-almost-equal-characters/
//
// Medium
//
// You are given a 0-indexed string word.
//
// In one operation, you can pick any index i of word and change word[i] to any lowercase English letter.
//
// Return the minimum number of operations needed to remove all adjacent almost-equal characters from word.
//
// Two characters a and b are almost-equal if a == b or a and b are adjacent in the alphabet.
//
// Example 1:
//
// Input: word = "aaaaa"
// Output: 2
// Explanation: We can change word into "acaca" which does not have any adjacent almost-equal characters.
// It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 2.
//
// Example 2:
//
// Input: word = "abddez"
// Output: 2
// Explanation: We can change word into "ybdoez" which does not have any adjacent almost-equal characters.
// It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 2.
//
// Example 3:
//
// Input: word = "zyxyxyz"
// Output: 3
// Explanation: We can change word into "zaxaxaz" which does not have any adjacent almost-equal characters.
// It can be shown that the minimum number of operations needed to remove all adjacent almost-equal characters from word is 3.
//
// Constraints:
//
//     1 <= word.length <= 100
//     word consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let s = word.as_bytes();
        while i < s.len() - 1 {
            if (s[i] as i32 - s[i + 1] as i32).abs() <= 1 {
                ans += 1;
                i += 1;
            }
            i += 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_almost_equal_characters("aaaaa".to_string()), 2);
    assert_eq!(Solution::remove_almost_equal_characters("abddez".to_string()), 2);
    assert_eq!(Solution::remove_almost_equal_characters("zyxyxyz".to_string()), 3);
}
