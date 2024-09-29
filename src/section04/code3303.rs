#![allow(dead_code)]

// 3303. Find the Occurrence of First Almost Equal Substring
// https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring/
// https://leetcode.cn/problems/find-the-occurrence-of-first-almost-equal-substring/
//
// Hard
//
// You are given two strings s and pattern.
//
// A string x is called almost equal to y if you can change at most one character in x to make it identical to y.
//
// Return the smallest starting index of a substring in s that is almost equal to pattern. If no such index exists, return -1.
// A substring is a contiguous non-empty sequence of characters within a string.
//
// Example 1:
//
// Input: s = "abcdefg", pattern = "bcdffg"
//
// Output: 1
//
// Explanation:
//
// The substring s[1..6] == "bcdefg" can be converted to "bcdffg" by changing s[4] to "f".
//
// Example 2:
//
// Input: s = "ababbababa", pattern = "bacaba"
//
// Output: 4
//
// Explanation:
//
// The substring s[4..9] == "bababa" can be converted to "bacaba" by changing s[6] to "c".
//
// Example 3:
//
// Input: s = "abcd", pattern = "dba"
//
// Output: -1
//
// Example 4:
//
// Input: s = "dde", pattern = "d"
//
// Output: 0
//
// Constraints:
//
//     1 <= pattern.length < s.length <= 3 * 10^5
//     s and pattern consist only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let n = s.len();
        let m = pattern.len();
        let pattern2 = pattern.chars().rev().collect::<String>();
        let s2 = s.chars().rev().collect::<String>();
        let z1 = Self::z_function(format!("{}{}", pattern, s));
        let z2 = Self::z_function(format!("{}{}", pattern2, s2));
        for i in 0..=n - m {
            if z1[m + i] + 1 + z2[n - i] >= m as i32 {
                return i as i32;
            }
        }
        -1
    }

    fn z_function(s: String) -> Vec<i32> {
        let n = s.len() as i32;
        let s = s.chars().collect::<Vec<char>>();
        let mut z = vec![0_i32; n as usize];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i <= r {
                z[i as usize] = z[(i - l) as usize].min(r - i + 1);
            }
            while i + z[i as usize] < n && s[z[i as usize] as usize] == s[(i + z[i as usize]) as usize] {
                z[i as usize] += 1;
            }
            if i + z[i as usize] - 1 > r {
                l = i;
                r = i + z[i as usize] - 1;
            }
        }
        z
    }
}

#[test]
fn test() {
    let s = "abcdefg".to_string();
    let pattern = "bcdffg".to_string();
    assert_eq!(Solution::min_starting_index(s, pattern), 1);

    let s = "ababbababa".to_string();
    let pattern = "bacaba".to_string();
    assert_eq!(Solution::min_starting_index(s, pattern), 4);

    let s = "abcd".to_string();
    let pattern = "dba".to_string();
    assert_eq!(Solution::min_starting_index(s, pattern), -1);

    let s = "dde".to_string();
    let pattern = "d".to_string();
    assert_eq!(Solution::min_starting_index(s, pattern), 0);
}
