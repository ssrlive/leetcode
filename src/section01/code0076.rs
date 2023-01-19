#![allow(dead_code)]

// 76. Minimum Window Substring
// https://leetcode.com/problems/minimum-window-substring/
// https://leetcode.cn/problems/minimum-window-substring/
//
// Given two strings s and t of lengths m and n respectively, return the minimum window substring
// of s such that every character in t (including duplicates) is included in the window.
// If there is no such substring, return the empty string "".
//
// The testcases will be generated such that the answer is unique.
//
// Example 1:
//
// Input: s = "ADOBECODEBANC", t = "ABC"
// Output: "BANC"
// Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
//
// Example 2:
//
// Input: s = "a", t = "a"
// Output: "a"
// Explanation: The entire string s is the minimum window.
//
// Example 3:
//
// Input: s = "a", t = "aa"
// Output: ""
// Explanation: Both 'a's from t must be included in the window.
// Since the largest window of s only has one 'a', return empty string.
//
// Constraints:
//
// - m == s.length
// - n == t.length
// - 1 <= m, n <= 10^5
// - s and t consist of uppercase and lowercase English letters.
//
// Follow up: Could you find an algorithm that runs in O(m + n) time?
//

struct Solution;

impl Solution {
    pub fn min_window_2(s: String, t: String) -> String {
        let mut map = [0; 58];
        let mut n = 0_usize;
        for c in t.chars() {
            n += 1;
            map[(c as u8 - b'A') as usize] += 1;
        }
        let mut min = 10000000_usize;
        let mut candidate: Option<(usize, usize)> = None;
        let char_list = s.as_bytes();
        let mut i = 0_usize;
        let mut count = 0;
        for j in 0..char_list.len() {
            if map[(char_list[j] - b'A') as usize] > 0 {
                count += 1;
            }
            map[(char_list[j] - b'A') as usize] -= 1;
            if count >= n && min > j - i {
                min = j - i;
                candidate = Some((i, j));
            }
            while i <= j && count >= n {
                if count >= n && min > j - i {
                    min = j - i;
                    candidate = Some((i, j));
                }
                map[(char_list[i] - b'A') as usize] += 1;
                if map[(char_list[i] - b'A') as usize] > 0 {
                    count -= 1;
                }
                i += 1;
            }
        }
        match candidate {
            Some((a, b)) => s[a..b + 1].to_string(),
            None => "".to_string(),
        }
    }

    pub fn min_window(s: String, t: String) -> String {
        let (mut c, b, mut ans, mut mi) = ([0; 60], s.as_bytes(), (0, 0), 1000000);
        for x in t.chars() {
            c[x as usize - 65] -= 1;
        }
        let (mut cnt, mut l) = (c.iter().filter(|&x| *x < 0).count(), 0);
        for (r, &x) in b.iter().enumerate() {
            c[x as usize - 65] += 1;
            if c[x as usize - 65] == 0 {
                cnt -= 1;
            }
            while cnt == 0 {
                if r - l + 1 < mi {
                    mi = r - l + 1;
                    ans = (l, r + 1);
                }
                if c[b[l] as usize - 65] == 0 {
                    cnt += 1;
                }
                c[b[l] as usize - 65] -= 1;
                l += 1;
            }
        }
        s[ans.0..ans.1].to_string()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("ADOBECODEBANC", "ABC", "BANC"),
        ("a", "a", "a"),
        ("a", "aa", ""),
        ("a", "b", ""),
        ("aa", "aa", "aa"),
    ];
    for (s, t, expect) in cases {
        assert_eq!(Solution::min_window(s.to_string(), t.to_string()), expect);
    }
}
