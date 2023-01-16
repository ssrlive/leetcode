#![allow(dead_code)]

// 828. Count Unique Characters of All Substrings of a Given String
// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/
// https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string/
//
// Let's define a function countUniqueChars(s) that returns the number of unique characters on s.
//
// For example, calling countUniqueChars(s) if s = "LEETCODE" then "L", "T", "C", "O", "D"
// are the unique characters since they appear only once in s, therefore countUniqueChars(s) = 5.
//
// Given a string s, return the sum of countUniqueChars(t) where t is a substring of s.
// The test cases are generated such that the answer fits in a 32-bit integer.
//
// Notice that some substrings can be repeated so in this case you have to count the repeated ones too.
//
// Example 1:
//
// Input: s = "ABC"
// Output: 10
// Explanation: All possible substrings are: "A","B","C","AB","BC" and "ABC".
// Every substring is composed with only unique letters.
// Sum of lengths of all substring is 1 + 1 + 1 + 2 + 2 + 3 = 10
//
// Example 2:
//
// Input: s = "ABA"
// Output: 8
// Explanation: The same as example 1, except countUniqueChars("ABA") = 1.
//
// Example 3:
//
// Input: s = "LEETCODE"
// Output: 92
//
// Constraints:
//
// - 1 <= s.length <= 10^5
// - s consists of uppercase English letters only.
//

struct Solution;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut last = vec![vec![-1; 26]; 2];
        let mut ret = 0;
        for (i, c) in s.bytes().enumerate() {
            let i = i as i32;
            let c = (c - b'A') as usize;
            ret += (i - last[0][c]) * (last[0][c] - last[1][c]);
            last[1][c] = last[0][c];
            last[0][c] = i;
        }
        for c in 0..26 {
            ret += (s.len() as i32 - last[0][c]) * (last[0][c] - last[1][c]);
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
    assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
    assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
}
