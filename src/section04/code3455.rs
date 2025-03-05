#![allow(dead_code)]

// 3455. Shortest Matching Substring
// https://leetcode.com/problems/shortest-matching-substring/
// https://leetcode.cn/problems/shortest-matching-substring/
//
// Hard
//
// You are given a string s and a pattern string p, where p contains exactly two '*' characters.
//
// The '*' in p matches any sequence of zero or more characters.
//
// Return the length of the shortest substring in s that matches p. If there is no such substring, return -1.
// Note: The empty substring is considered valid.
//
// Example 1:
//
// Input: s = "abaacbaecebce", p = "ba*c*ce"
//
// Output: 8
//
// Explanation:
//
// The shortest matching substring of p in s is "baecebce".
//
// Example 2:
//
// Input: s = "baccbaadbc", p = "cc*baa*adb"
//
// Output: -1
//
// Explanation:
//
// There is no matching substring in s.
//
// Example 3:
//
// Input: s = "a", p = "**"
//
// Output: 0
//
// Explanation:
//
// The empty substring is the shortest matching substring.
//
// Example 4:
//
// Input: s = "madlogic", p = "*adlogi*"
//
// Output: 6
//
// Explanation:
//
// The shortest matching substring of p in s is "adlogi".
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     2 <= p.length <= 10^5
//     s contains only lowercase English letters.
//     p contains only lowercase English letters and exactly two '*'.
//

struct Solution;

impl Solution {
    pub fn shortest_matching_substring(s: String, p: String) -> i32 {
        fn kmp(s: &str) -> Vec<usize> {
            let s = s.as_bytes();
            let n = s.len();
            let mut lps = vec![0; n + 1];
            for i in 1..n {
                let mut prev = lps[i - 1];
                while prev > 0 && s[i] != s[prev] {
                    prev = lps[prev - 1];
                }
                lps[i] = prev + (s[i] == s[prev]) as usize;
            }
            lps
        }

        let _n = s.len();
        let m = p.len();
        if m == 2 {
            return 0;
        }
        let mut v = Vec::new();
        for (i, c) in p.chars().enumerate() {
            if c == '*' {
                v.push(i);
            }
        }
        let a = &p[0..v[0]];
        let b = &p[v[0] + 1..v[1]];
        let c = &p[v[1] + 1..m];
        let na = a.len();
        let nb = b.len();
        let nc = c.len();
        let v1 = kmp(&(a.to_string() + "#" + &s));
        let v2 = kmp(&(b.to_string() + "#" + &s));
        let v3 = kmp(&(c.to_string() + "#" + &s));

        let v1 = v1.split_at(na + 1).1.to_vec();
        let v2 = v2.split_at(nb + 1).1.to_vec();
        let v3 = v3.split_at(nc + 1).1.to_vec();
        let mut res = i32::MAX;
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i + nb + nc < v1.len() {
            while i < v1.len() && v1[i] != na {
                i += 1;
            }
            if i >= v1.len() {
                break;
            }
            while j < v2.len() && (j < i + nb || v2[j] != nb) {
                j += 1;
            }
            if j >= v2.len() {
                break;
            }
            while k < v3.len() && (k < j + nc || v3[k] != nc) {
                k += 1;
            }
            if k >= v3.len() {
                break;
            }
            res = res.min((k - i + na) as i32);
            i += 1;
        }
        if res == i32::MAX { -1 } else { res }
    }
}

#[test]
fn test() {
    let s = "abaacbaecebce".to_string();
    let p = "ba*c*ce".to_string();
    let output = 8;
    assert_eq!(Solution::shortest_matching_substring(s, p), output);

    let s = "baccbaadbc".to_string();
    let p = "cc*baa*adb".to_string();
    let output = -1;
    assert_eq!(Solution::shortest_matching_substring(s, p), output);

    let s = "a".to_string();
    let p = "**".to_string();
    let output = 0;
    assert_eq!(Solution::shortest_matching_substring(s, p), output);

    let s = "madlogic".to_string();
    let p = "*adlogi*".to_string();
    let output = 6;
    assert_eq!(Solution::shortest_matching_substring(s, p), output);
}
