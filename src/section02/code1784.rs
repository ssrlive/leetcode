#![allow(dead_code)]

/*

// 1784. Check if Binary String Has at Most One Segment of Ones
Easy
252
764
Companies

Given a binary string s ​​​​​without leading zeros, return true​​​ if s contains at most one contiguous segment of ones. Otherwise, return false.

Example 1:

Input: s = "1001"
Output: false
Explanation: The ones do not form a contiguous segment.

Example 2:

Input: s = "110"
Output: true

Constraints:

    1 <= s.length <= 100
    s[i]​​​​ is either '0' or '1'.
    s[0] is '1'.
*/

struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.split('0').filter(|x| !x.is_empty()).count() == 1
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("1001", false),
        ("110", true),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::check_ones_segment(s.to_string()), expected);
    }
}
