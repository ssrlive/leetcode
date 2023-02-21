#![allow(dead_code)]

/*

// 1796. Second Largest Digit in a String
Easy
390
101
Companies

Given an alphanumeric string s, return the second largest numerical digit that appears in s, or -1 if it does not exist.

An alphanumeric string is a string consisting of lowercase English letters and digits.

Example 1:

Input: s = "dfa12321afd"
Output: 2
Explanation: The digits that appear in s are [1, 2, 3]. The second largest digit is 2.

Example 2:

Input: s = "abc1111"
Output: -1
Explanation: The digits that appear in s are [1]. There is no second largest digit.

Constraints:

    1 <= s.length <= 500
    s consists of only lowercase English letters and/or digits.
*/

struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut ar = [false; 10];

        s.as_bytes()
            .iter()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x - b'0')
            .for_each(|x| ar[x as usize] = true);

        ar.iter()
            .enumerate()
            .filter_map(|(ind, val)| if *val { Some(ind as i32) } else { None })
            .rev()
            .nth(1)
            .unwrap_or(-1)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("dfa12321afd", 2),
        ("abc1111", -1),
        ("ck0", -1),
        ("ck077", 0),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::second_highest(s.to_string()), expected);
    }
}
