#![allow(dead_code)]

// 3297. Count Substrings That Can Be Rearranged to Contain a String I
// https://leetcode.com/problems/count-substrings-that-can-be-rearranged-to-contain-a-string-i/
// https://leetcode.cn/problems/count-substrings-that-can-be-rearranged-to-contain-a-string-i/
//
// Medium
//
// You are given two strings word1 and word2.
//
// A string x is called valid if x can be rearranged to have word2 as a prefix.
//
// Return the total number of valid substrings of word1.
//
// Example 1:
//
// Input: word1 = "bcca", word2 = "abc"
//
// Output: 1
//
// Explanation:
//
// The only valid substring is "bcca" which can be rearranged to "abcc" having "abc" as a prefix.
//
// Example 2:
//
// Input: word1 = "abcabc", word2 = "abc"
//
// Output: 10
//
// Explanation:
//
// All the substrings except substrings of size 1 and size 2 are valid.
//
// Example 3:
//
// Input: word1 = "abcabc", word2 = "aaabc"
//
// Output: 0
//
// Constraints:
//
// 1 <= word1.length <= 10^5
// 1 <= word2.length <= 10^4
// word1 and word2 consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut cnt = [0; 26];
        let mut j = 0;
        let mut match_ = 0;
        let mut res = 0;
        for ch in word2.chars() {
            match_ += if cnt[ch as usize - 'a' as usize] == 0 { 1 } else { 0 };
            cnt[ch as usize - 'a' as usize] += 1;
        }
        let word1 = word1.as_bytes();
        for (i, &ch) in word1.iter().enumerate() {
            cnt[ch as usize - 'a' as usize] -= 1;
            match_ -= if cnt[ch as usize - 'a' as usize] == 0 { 1 } else { 0 };
            while match_ == 0 {
                res += (word1.len() - i) as i64;
                match_ += if cnt[word1[j] as usize - 'a' as usize] == 0 { 1 } else { 0 };
                cnt[word1[j] as usize - 'a' as usize] += 1;
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::valid_substring_count("bcca".to_string(), "abc".to_string()), 1);
    assert_eq!(Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()), 10);
    assert_eq!(Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()), 0);
}
