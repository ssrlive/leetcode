#![allow(dead_code)]

/*

// 1844. Replace All Digits with Characters
// https://leetcode.com/problems/replace-all-digits-with-characters/
// https://leetcode.cn/problems/replace-all-digits-with-characters/
//
// Easy
//
// You are given a 0-indexed string s that has lowercase English letters in its even indices and digits in its odd indices.

There is a function shift(c, x), where c is a character and x is a digit, that returns the xth character after c.

    For example, shift('a', 5) = 'f' and shift('x', 0) = 'x'.

For every odd index i, you want to replace the digit s[i] with shift(s[i-1], s[i]).

Return s after replacing all digits. It is guaranteed that shift(s[i-1], s[i]) will never exceed 'z'.

Example 1:

Input: s = "a1c1e1"
Output: "abcdef"
Explanation: The digits are replaced as follows:
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('c',1) = 'd'
- s[5] -> shift('e',1) = 'f'

Example 2:

Input: s = "a1b2c3d4e"
Output: "abbdcfdhe"
Explanation: The digits are replaced as follows:
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('b',2) = 'd'
- s[5] -> shift('c',3) = 'f'
- s[7] -> shift('d',4) = 'h'

Constraints:

    1 <= s.length <= 100
    s consists only of lowercase English letters and digits.
    shift(s[i-1], s[i]) <= 'z' for all odd indices i.
*/

struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut prev_b = b'a';
        String::from_utf8(
            s.as_bytes()
                .iter()
                .map(|&cur_b| match cur_b >= b'a' {
                    true => {
                        prev_b = cur_b;
                        cur_b
                    }
                    false => prev_b + (cur_b - b'0'),
                })
                .collect(),
        )
        .unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("a1c1e1", "abcdef"),
        ("a1b2c3d4e", "abbdcfdhe"),
        ("a1b2c3d4e5", "abbdcfdhej"),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::replace_digits(s.to_string()), expect);
    }
}
