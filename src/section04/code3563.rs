#![allow(dead_code)]

// 3563. Lexicographically Smallest String After Adjacent Removals
// https://leetcode.com/problems/lexicographically-smallest-string-after-adjacent-removals/
// https://leetcode.cn/problems/lexicographically-smallest-string-after-adjacent-removals/
//
// Hard
//
// You are given a string s consisting of lowercase English letters.
//
// You can perform the following operation any number of times (including zero):
//
//     Remove any pair of adjacent characters in the string that are consecutive in the alphabet, in either order (e.g., 'a' and 'b', or 'b' and 'a').
//     Shift the remaining characters to the left to fill the gap.
//
// Return the string that can be obtained after performing the operations optimally.
//
// Note: Consider the alphabet as circular, thus 'a' and 'z' are consecutive.
//
// Example 1:
//
// Input: s = "abc"
//
// Output: "a"
//
// Explanation:
//
//     Remove "bc" from the string, leaving "a" as the remaining string.
//     No further operations are possible. Thus, the lexicographically smallest string after all possible removals is "a".
//
// Example 2:
//
// Input: s = "bcda"
//
// Output: ""
//
// Explanation:
//
//     ​​​​​​​Remove "cd" from the string, leaving "ba" as the remaining string.
//     Remove "ba" from the string, leaving "" as the remaining string.
//     No further operations are possible. Thus, the lexicographically smallest string after all possible removals is "".
//
// Example 3:
//
// Input: s = "zdce"
//
// Output: "zdce"
//
// Explanation:
//
//     Remove "dc" from the string, leaving "ze" as the remaining string.
//     No further operations are possible on "ze".
//     However, since "zdce" is lexicographically smaller than "ze", the smallest string after all possible removals is "zdce".
//
// Constraints:
//
//     1 <= s.length <= 250
//     s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_string(s: String) -> String {
        fn is_consec(a: char, b: char) -> bool {
            (a as u8).abs_diff(b as u8) == 1 || (a as u8).abs_diff(b as u8) == 25
        }

        let n = s.len() as i32;
        let mut dp = vec![vec![String::new(); (n + 1) as usize]; (n + 1) as usize];
        let s: Vec<char> = s.chars().collect();
        for len in 1..=n {
            for i in 0..=n - len {
                let j = i + len;
                let mut res = s[i as usize].to_string() + &dp[(i + 1) as usize][j as usize];

                for k in (i + 1)..j {
                    if is_consec(s[i as usize], s[k as usize]) && dp[(i + 1) as usize][k as usize].is_empty() {
                        res = res.min(dp[(k + 1) as usize][j as usize].clone());
                    }
                }

                dp[i as usize][j as usize] = res;
            }
        }
        dp[0][n as usize].clone()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::lexicographically_smallest_string("abc".to_string()), "a");
    assert_eq!(Solution::lexicographically_smallest_string("bcda".to_string()), "");
    assert_eq!(Solution::lexicographically_smallest_string("zdce".to_string()), "zdce");
}
