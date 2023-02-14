#![allow(dead_code)]

/*

// 1625. Lexicographically Smallest String After Applying Operations
// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/
// https://leetcode.cn/problems/lexicographically-smallest-string-after-applying-operations/
//
// Medium
//
// You are given a string s of even length consisting of digits from 0 to 9, and two integers a and b.

You can apply either of the following two operations any number of times and in any order on s:

    Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example, if s = "3456" and a = 5, s becomes "3951".
    Rotate s to the right by b positions. For example, if s = "3456" and b = 1, s becomes "6345".

Return the lexicographically smallest string you can obtain by applying the above operations any number of times on s.

A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b. For example, "0158" is lexicographically smaller than "0190" because the first position they differ is at the third letter, and '5' comes before '9'.

Example 1:

Input: s = "5525", a = 9, b = 2
Output: "2050"
Explanation: We can apply the following operations:
Start:  "5525"
Rotate: "2555"
Add:    "2454"
Add:    "2353"
Rotate: "5323"
Add:    "5222"
Add:    "5121"
Rotate: "2151"
​​​​​​​Add:    "2050"​​​​​​​​​​​​
There is no way to obtain a string that is lexicographically smaller then "2050".

Example 2:

Input: s = "74", a = 5, b = 1
Output: "24"
Explanation: We can apply the following operations:
Start:  "74"
Rotate: "47"
​​​​​​​Add:    "42"
​​​​​​​Rotate: "24"​​​​​​​​​​​​
There is no way to obtain a string that is lexicographically smaller then "24".

Example 3:

Input: s = "0011", a = 4, b = 2
Output: "0011"
Explanation: There are no sequence of operations that will give us a lexicographically smaller string than "0011".

Constraints:

    2 <= s.length <= 100
    s.length is even.
    s consists of digits from 0 to 9 only.
    1 <= a <= 9
    1 <= b <= s.length - 1
*/

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        use std::{cmp::min, collections::HashSet};

        fn rotate(s: &str, b: i32) -> String {
            let mut res = String::new();
            let i: usize = s.len() - b as usize;
            res += &s[i..];
            res += &s[..i];
            res
        }

        fn add(s: &str, a: i32) -> String {
            let mut res = String::new();
            for (i, c) in s.chars().enumerate() {
                if i % 2 == 0 {
                    res.push(c);
                } else {
                    res.push((b'0' + (c as u8 - b'0' + a as u8) % 10) as char);
                }
            }
            res
        }

        fn dfs(s: &str, a: i32, b: i32, seen: &mut HashSet<String>) -> Option<String> {
            if !seen.contains(s) {
                let mut res = s.to_string();
                seen.insert(s.to_string());
                if let Some(ss) = dfs(&rotate(s, b), a, b, seen) {
                    res = min(res, ss);
                }
                if let Some(ss) = dfs(&add(s, a), a, b, seen) {
                    res = min(res, ss);
                }
                Some(res)
            } else {
                None
            }
        }

        let mut seen = HashSet::new();
        dfs(&s, a, b, &mut seen).unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![("5525", 9, 2, "2050"), ("74", 5, 1, "24"), ("0011", 4, 2, "0011")];
    for (s, a, b, expect) in cases {
        assert_eq!(Solution::find_lex_smallest_string(s.to_string(), a, b), expect);
    }
}
