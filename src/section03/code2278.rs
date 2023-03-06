#![allow(dead_code)]

/*

// 2278. Percentage of Letter in String
// https://leetcode.com/problems/percentage-of-letter-in-string/
// https://leetcode.cn/problems/percentage-of-letter-in-string/
//
// Easy
//
// Given a string s and a character letter, return the percentage of characters in s that equal letter rounded down to the nearest whole percent.

Example 1:

Input: s = "foobar", letter = "o"
Output: 33
Explanation:
The percentage of characters in s that equal the letter 'o' is 2 / 6 * 100% = 33% when rounded down, so we return 33.

Example 2:

Input: s = "jjjj", letter = "k"
Output: 0
Explanation:
The percentage of characters in s that equal the letter 'k' is 0%, so we return 0.

Constraints:

    1 <= s.length <= 100
    s consists of lowercase English letters.
    letter is a lowercase English letter.
*/

struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let len_s: usize = s.len();
        let cnt: usize = s.chars().filter(|&c| c == letter).count();
        (cnt as i32 * 100) / len_s as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
    assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
}
