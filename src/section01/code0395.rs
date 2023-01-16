#![allow(dead_code)]

// 395. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
//
// Given a string s and an integer k, return the length of the longest substring of s such that the frequency
// of each character in this substring is greater than or equal to k.
//
// Example 1:
//
// Input: s = "aaabb", k = 3
// Output: 3
// Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
//
// Example 2:
//
// Input: s = "ababbc", k = 2
// Output: 5
// Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - s consists of only lowercase English letters.
// - 1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut res = 0;
        for i in 1..=26 {
            let mut left = 0;
            let mut right = 0;
            let mut count = vec![0; 26];
            let mut unique = 0;
            let mut no_less_than_k = 0;
            while right < s.len() {
                if unique <= i {
                    let idx = (s.as_bytes()[right] - b'a') as usize;
                    if count[idx] == 0 {
                        unique += 1;
                    }
                    count[idx] += 1;
                    if count[idx] == k {
                        no_less_than_k += 1;
                    }
                    right += 1;
                } else {
                    let idx = (s.as_bytes()[left] - b'a') as usize;
                    if count[idx] == k {
                        no_less_than_k -= 1;
                    }
                    count[idx] -= 1;
                    if count[idx] == 0 {
                        unique -= 1;
                    }
                    left += 1;
                }
                if unique == i && unique == no_less_than_k {
                    res = res.max(right - left);
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
    assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
}
