#![allow(dead_code)]

// 2472. Maximum Number of Non-overlapping Palindrome Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/
// https://leetcode.cn/problems/maximum-number-of-non-overlapping-palindrome-substrings/
//
// You are given a string s and a positive integer k.
//
// Select a set of non-overlapping substrings from the string s that satisfy the following conditions:
//
// - The length of each substring is at least k.
// - Each substring is a palindrome.
//
// Return the maximum number of substrings in an optimal selection.
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: s = "abaccdbbd", k = 3
// Output: 2
// Explanation: We can select the substrings underlined in s = "abaccdbbd".
//              Both "aba" and "dbbd" are palindromes and have a length of at least k = 3.
//              It can be shown that we cannot find a selection with more than two valid substrings.
//
// Example 2:
//
// Input: s = "adbcda", k = 2
// Output: 0
// Explanation: There is no palindrome substring of length at least 2 in the string.
//
// Constraints:
//
// - 1 <= k <= s.length <= 2000
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        fn collect(s: &str, k: usize) -> Vec<(usize, usize)> {
            let n = s.len();
            let s = s.chars().collect::<Vec<char>>();
            let mut ret = vec![];
            for i in 0..n {
                let (mut left, mut right) = (i, i);
                while left > 0 && right + 1 < n && right - left + 1 < k && s[left - 1] == s[right + 1] {
                    left -= 1;
                    right += 1;
                }
                if right - left + 1 >= k {
                    ret.push((left, right));
                }

                let (mut left, mut right) = (i, i + 1);
                if right == n || s[left] != s[right] {
                    continue;
                }

                while left > 0 && right + 1 < n && right - left + 1 < k && s[left - 1] == s[right + 1] {
                    left -= 1;
                    right += 1;
                }
                if right - left + 1 >= k {
                    ret.push((left, right));
                }
            }

            ret
        }

        let mut data = collect(&s, k as usize);
        data.sort();
        data.reverse();

        let (mut count, mut last) = (0, 0);
        while !data.is_empty() {
            while !data.is_empty() && last > 0 && data[data.len() - 1].0 <= last {
                data.pop();
            }
            if data.is_empty() {
                continue;
            }
            count += 1;
            last = data[data.len() - 1].1;
            data.pop();
        }

        count
    }
}
