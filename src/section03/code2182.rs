#![allow(dead_code)]

/*

// 2182. Construct String With Repeat Limit
// https://leetcode.com/problems/construct-string-with-repeat-limit/
// https://leetcode.cn/problems/construct-string-with-repeat-limit/
//
// Medium
//
// You are given a string s and an integer repeatLimit. Construct a new string repeatLimitedString using the characters of s such that no letter appears more than repeatLimit times in a row. You do not have to use all characters from s.

Return the lexicographically largest repeatLimitedString possible.

A string a is lexicographically larger than a string b if in the first position where a and b differ, string a has a letter that appears later in the alphabet than the corresponding letter in b. If the first min(a.length, b.length) characters do not differ, then the longer string is the lexicographically larger one.

Example 1:

Input: s = "cczazcc", repeatLimit = 3
Output: "zzcccac"
Explanation: We use all of the characters from s to construct the repeatLimitedString "zzcccac".
The letter 'a' appears at most 1 time in a row.
The letter 'c' appears at most 3 times in a row.
The letter 'z' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "zzcccac".
Note that the string "zzcccca" is lexicographically larger but the letter 'c' appears more than 3 times in a row, so it is not a valid repeatLimitedString.

Example 2:

Input: s = "aababab", repeatLimit = 2
Output: "bbabaa"
Explanation: We use only some of the characters from s to construct the repeatLimitedString "bbabaa".
The letter 'a' appears at most 2 times in a row.
The letter 'b' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "bbabaa".
Note that the string "bbabaaa" is lexicographically larger but the letter 'a' appears more than 2 times in a row, so it is not a valid repeatLimitedString.

Constraints:

    1 <= repeatLimit <= s.length <= 10^5
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut memo = [0; 26];
        for c in s.chars() {
            memo[(c as u8 - b'a') as usize] += 1;
        }
        let mut result = vec![];
        let mut i = 25;
        let mut temp = 0;
        loop {
            if memo[i] == 0 {
                if i == 0 {
                    break;
                }
                i -= 1;
                temp = 0;
                continue;
            }
            if temp < repeat_limit {
                result.push((i as u8 + b'a') as char);
                memo[i] -= 1;
                temp += 1;
            } else {
                let mut success = false;
                if i == 0 {
                    break;
                }
                for j in (0..=i - 1).rev() {
                    if memo[j] != 0 {
                        memo[j] -= 1;
                        temp = 0;
                        result.push((j as u8 + b'a') as char);
                        success = true;
                        break;
                    }
                }
                if !success {
                    break;
                }
            }
        }
        result.into_iter().map(|v| v.to_string()).collect()
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("cczazcc", 3, "zzcccac"),
        ("aababab", 2, "bbabaa"),
    ];
    for (s, repeat_limit, expected) in cases {
        assert_eq!(Solution::repeat_limited_string(s.to_string(), repeat_limit), expected);
    }
}
