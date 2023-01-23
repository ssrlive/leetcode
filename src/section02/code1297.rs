#![allow(dead_code)]

// 1297. Maximum Number of Occurrences of a Substring
// https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/
// https://leetcode.cn/problems/maximum-number-of-occurrences-of-a-substring/
//
// Medium
//
// Given a string s, return the maximum number of ocurrences of any substring under the following rules:
//
//     The number of unique characters in the substring must be less than or equal to maxLetters.
//     The substring size must be between minSize and maxSize inclusive.
//
// Example 1:
//
// Input: s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
// Output: 2
// Explanation: Substring "aab" has 2 ocurrences in the original string.
// It satisfies the conditions, 2 unique letters and size 3 (between minSize and maxSize).
//
// Example 2:
//
// Input: s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
// Output: 2
// Explanation: Substring "aaa" occur 2 times in the string. It can overlap.
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    1 <= maxLetters <= 26
// -    1 <= minSize <= maxSize <= min(26, s.length)
// -    s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        use std::collections::*;
        let ss = s.clone();
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut map = HashMap::new();

        let min = min_size as usize;
        let max = max_size as usize;
        for i in min..=max {
            let mut memo = vec![0; 26];
            let mut count = 0;
            for &item in s.iter().take(i) {
                let ci = (item as u8 - b'a') as usize;
                if memo[ci] == 0 {
                    count += 1;
                }
                memo[ci] += 1;
            }

            if count <= max_letters {
                *map.entry(&ss[0..i]).or_insert(0) += 1;
            }

            for j in i..n {
                let li = (s[j - i] as u8 - b'a') as usize;
                memo[li] -= 1;
                if memo[li] == 0 {
                    count -= 1;
                }

                let ci = (s[j] as u8 - b'a') as usize;
                if memo[ci] == 0 {
                    count += 1;
                }
                memo[ci] += 1;

                if count <= max_letters {
                    *map.entry(&ss[j - i + 1..=j]).or_insert(0) += 1;
                }
            }
        }

        let mut max = 0;
        for (_, num) in map {
            max = max.max(num);
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aababcaab", 2, 3, 4, 2),
        ("aaaa", 1, 3, 3, 2),
        ("aabcabcab", 2, 2, 3, 3),
        ("abcde", 2, 3, 3, 0),
    ];
    for (s, max_letters, min_size, max_size, expect) in cases {
        assert_eq!(
            Solution::max_freq(s.to_string(), max_letters, min_size, max_size),
            expect
        );
    }
}
