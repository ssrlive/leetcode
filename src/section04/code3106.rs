#![allow(dead_code)]

// 3106. Lexicographically Smallest String After Operations With Constraint
// https://leetcode.com/problems/lexicographically-smallest-string-after-operations-with-constraint/
// https://leetcode.cn/problems/lexicographically-smallest-string-after-operations-with-constraint/
//
// Medium
//
// You are given a string s and an integer k.
//
// Define a function distance(s1, s2) between two strings s1 and s2 of the same length n as:
//
// - The sum of the minimum distance between s1[i] and s2[i] when the characters from 'a' to 'z'
//   are placed in a cyclic order, for all i in the range [0, n - 1].
//
// For example, distance("ab", "cd") == 4, and distance("a", "z") == 1.
//
// You can change any letter of s to any other lowercase English letter, any number of times.
//
// Return a string denoting the lexicographically smallest string t you can get after some changes, such that distance(s, t) <= k.
//
// Example 1:
//
// Input: s = "zbbz", k = 3
//
// Output: "aaaz"
//
// Explanation:
//
// Change s to "aaaz". The distance between "zbbz" and "aaaz" is equal to k = 3.
//
// Example 2:
//
// Input: s = "xaxcd", k = 4
//
// Output: "aawcd"
//
// Explanation:
//
// The distance between "xaxcd" and "aawcd" is equal to k = 4.
//
// Example 3:
//
// Input: s = "lol", k = 0
//
// Output: "lol"
//
// Explanation:
//
// It's impossible to change any character as k = 0.
//
// Constraints:
//
//     1 <= s.length <= 100
//     0 <= k <= 2000
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut ans = String::new();
        let (n, mut k) = (s.len(), k);
        let s = s.as_bytes();
        for &s_i in s.iter().take(n) {
            let mut f = false;
            for ch in b'a'..=b'z' {
                let d1 = (s_i as i32 - ch as i32).abs();
                let d2 = 26 - d1;
                let d = d1.min(d2);
                if d <= k {
                    f = true;
                    ans.push(ch as char);
                    k -= d;
                    break;
                }
            }
            if !f {
                ans.push(s_i as char);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let s = "zbbz".to_string();
    let k = 3;
    let output = "aaaz".to_string();
    assert_eq!(Solution::get_smallest_string(s, k), output);

    let s = "xaxcd".to_string();
    let k = 4;
    let output = "aawcd".to_string();
    assert_eq!(Solution::get_smallest_string(s, k), output);

    let s = "lol".to_string();
    let k = 0;
    let output = "lol".to_string();
    assert_eq!(Solution::get_smallest_string(s, k), output);
}
