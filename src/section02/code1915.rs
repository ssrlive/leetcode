#![allow(dead_code)]

/*

// 1915. Number of Wonderful Substrings
// https://leetcode.com/problems/number-of-wonderful-substrings/
// https://leetcode.cn/problems/number-of-wonderful-substrings/
//
// Medium
//
// A wonderful string is a string where at most one letter appears an odd number of times.

    For example, "ccjjc" and "abab" are wonderful, but "ab" is not.

Given a string word that consists of the first ten lowercase English letters ('a' through 'j'), return the number of wonderful non-empty substrings in word. If the same substring appears multiple times in word, then count each occurrence separately.

A substring is a contiguous sequence of characters in a string.

Example 1:

Input: word = "aba"
Output: 4
Explanation: The four wonderful substrings are underlined below:
- "aba" -> "a"
- "aba" -> "b"
- "aba" -> "a"
- "aba" -> "aba"

Example 2:

Input: word = "aabb"
Output: 9
Explanation: The nine wonderful substrings are underlined below:
- "aabb" -> "a"
- "aabb" -> "aa"
- "aabb" -> "aab"
- "aabb" -> "aabb"
- "aabb" -> "a"
- "aabb" -> "abb"
- "aabb" -> "b"
- "aabb" -> "bb"
- "aabb" -> "b"

Example 3:

Input: word = "he"
Output: 2
Explanation: The two wonderful substrings are underlined below:
- "he" -> "h"
- "he" -> "e"

Constraints:

    1 <= word.length <= 10^5
    word consists of lowercase English letters from 'a' to 'j'.
*/

struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let (mut cnt, mut mask, mut res) = (vec![0; 1024], 0, 0);
        cnt[0] = 1;
        for ch in word.chars() {
            mask ^= 1 << (ch as u8 - b'a');
            res += cnt[mask];
            for n in 0..10 {
                res += cnt[mask ^ (1 << n)];
            }
            cnt[mask] += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::wonderful_substrings("aba".to_string()), 4);
    assert_eq!(Solution::wonderful_substrings("aabb".to_string()), 9);
    assert_eq!(Solution::wonderful_substrings("he".to_string()), 2);
}
