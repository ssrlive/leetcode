#![allow(dead_code)]

/*

// 2309. Greatest English Letter in Upper and Lower Case
// https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/
// https://leetcode.cn/problems/greatest-english-letter-in-upper-and-lower-case/
//
// Easy
//
// Given a string of English letters s, return the greatest English letter which occurs
// as both a lowercase and uppercase letter in s. The returned letter should be in uppercase.
// If no such letter exists, return an empty string.

An English letter b is greater than another letter a if b appears after a in the English alphabet.

Example 1:

Input: s = "lEeTcOdE"
Output: "E"
Explanation:
The letter 'E' is the only letter to appear in both lower and upper case.

Example 2:

Input: s = "arRAzFif"
Output: "R"
Explanation:
The letter 'R' is the greatest letter to appear in both lower and upper case.
Note that 'A' and 'F' also appear in both lower and upper case, but 'R' is greater than 'F' or 'A'.

Example 3:

Input: s = "AbCdEfGhIjK"
Output: ""
Explanation:
There is no letter that appears in both lower and upper case.

Constraints:

    1 <= s.length <= 1000
    s consists of lowercase and uppercase English letters.
*/

struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut counter = [false; 58];
        s.bytes().for_each(|b| counter[(b - b'A') as usize] = true);
        match counter[..26]
            .iter()
            .enumerate()
            .filter(|&(i, &val)| val && counter[i + 32])
            .max()
        {
            Some((i, _)) => ((i as u8 + b'A') as char).to_string(),
            None => "".to_string(),
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("lEeTcOdE", "E"),
        ("arRAzFif", "R"),
        ("AbCdEfGhIjK", ""),
        ("aA", "A"),
        ("aAa", "A"),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::greatest_letter(s.to_string()), expect);
    }
}
