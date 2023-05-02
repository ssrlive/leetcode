#![allow(dead_code)]

/*
// 2663. Lexicographically Smallest Beautiful String
// https://leetcode.com/problems/lexicographically-smallest-beautiful-string/
// https://leetcode.cn/problems/lexicographically-smallest-beautiful-string/
//
// Hard
//
// A string is beautiful if:

    It consists of the first k letters of the English lowercase alphabet.
    It does not contain any substring of length 2 or more which is a palindrome.

You are given a beautiful string s of length n and a positive integer k.

Return the lexicographically smallest string of length n, which is larger than s and is beautiful. If there is no such string, return an empty string.

A string a is lexicographically larger than a string b (of the same length) if in the first position where a and b differ, a has a character strictly larger than the corresponding character in b.

    For example, "abcd" is lexicographically larger than "abcc" because the first position they differ is at the fourth character, and d is greater than c.

Example 1:

Input: s = "abcz", k = 26
Output: "abda"
Explanation: The string "abda" is beautiful and lexicographically larger than the string "abcz".
It can be proven that there is no string that is lexicographically larger than the string "abcz", beautiful, and lexicographically smaller than the string "abda".

Example 2:

Input: s = "dc", k = 4
Output: ""
Explanation: It can be proven that there is no string that is lexicographically larger than the string "dc" and is beautiful.

Constraints:

    1 <= n == s.length <= 10^5
    4 <= k <= 26
    s is a beautiful string.
*/

struct Solution;

impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let k = k as u8;

        fn valid(s: &[u8], i: usize) -> bool {
            (i < 1 || s[i] != s[i - 1]) && (i < 2 || s[i] != s[i - 2])
        }

        for i in (0..s.len()).rev() {
            s[i] += 1;
            while !valid(&s, i) {
                s[i] += 1;
            }
            if s[i] < b'a' + k {
                for i in i + 1..s.len() {
                    for c in b'a'..b'a' + k {
                        s[i] = c;
                        if valid(&s, i) {
                            break;
                        }
                    }
                }
                return String::from_utf8(s).unwrap();
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let s = "abcz".to_string();
    let k = 26;
    let res = "abda".to_string();
    assert_eq!(Solution::smallest_beautiful_string(s, k), res);

    let s = "dc".to_string();
    let k = 4;
    let res = "".to_string();
    assert_eq!(Solution::smallest_beautiful_string(s, k), res);
}
