#![allow(dead_code)]

// 1374. Generate a String With Characters That Have Odd Counts
// https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
// https://leetcode.cn/problems/generate-a-string-with-characters-that-have-odd-counts/
//
// Easy
//
// Given an integer n, return a string with n characters such that each character in such string occurs an odd number of times.
//
// The returned string must contain only lowercase English letters. If there are multiples valid strings, return any of them.
//
// Example 1:
//
// Input: n = 4
// Output: "pppz"
// Explanation: "pppz" is a valid string since the character 'p' occurs three times and the character 'z' occurs once.
// Note that there are many other valid strings such as "ohhh" and "love".
//
// Example 2:
//
// Input: n = 2
// Output: "xy"
// Explanation: "xy" is a valid string since the characters 'x' and 'y' occur once.
// Note that there are many other valid strings such as "ag" and "ur".
//
// Example 3:
//
// Input: n = 7
// Output: "holasss"
//
// Constraints:
//
// -    1 <= n <= 500
//

struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        std::iter::repeat_n('a', (n - 1 + n % 2) as usize)
            .chain(std::iter::once('b').take((1 - n % 2) as usize))
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::generate_the_string(4), "aaab".to_string());
    assert_eq!(Solution::generate_the_string(2), "ab".to_string());
    assert_eq!(Solution::generate_the_string(7), "aaaaaaa".to_string());
}
