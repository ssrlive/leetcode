#![allow(dead_code)]

/*

// 1869. Longer Contiguous Segments of Ones than Zeros
// https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/
// https://leetcode.cn/problems/longer-contiguous-segments-of-ones-than-zeros/
//
// Easy
//
// Given a binary string s, return true if the longest contiguous segment of 1's is strictly longer than the longest contiguous segment of 0's in s, or return false otherwise.

    For example, in s = "110100010" the longest continuous segment of 1s has length 2, and the longest continuous segment of 0s has length 3.

Note that if there are no 0's, then the longest continuous segment of 0's is considered to have a length 0. The same applies if there is no 1's.

Example 1:

Input: s = "1101"
Output: true
Explanation:
The longest contiguous segment of 1s has length 2: "1101"
The longest contiguous segment of 0s has length 1: "1101"
The segment of 1s is longer, so return true.

Example 2:

Input: s = "111000"
Output: false
Explanation:
The longest contiguous segment of 1s has length 3: "111000"
The longest contiguous segment of 0s has length 3: "111000"
The segment of 1s is not longer, so return false.

Example 3:

Input: s = "110100010"
Output: false
Explanation:
The longest contiguous segment of 1s has length 2: "110100010"
The longest contiguous segment of 0s has length 3: "110100010"
The segment of 1s is not longer, so return false.

Constraints:

    1 <= s.length <= 100
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let (mut max_1, mut max_0) = (0, 0);
        let (mut cur_1, mut cur_0) = (0, 0);
        s.chars().for_each(|c| match c == '1' {
            true => {
                cur_0 = 0;
                cur_1 += 1;
                max_1 = max_1.max(cur_1)
            }
            false => {
                cur_1 = 0;
                cur_0 += 1;
                max_0 = max_0.max(cur_0)
            }
        });
        max_1 > max_0
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("1101", true),
        ("111000", false),
        ("110100010", false),
    ];
    for (s, expected) in cases {
        assert_eq!(Solution::check_zero_ones(s.to_string()), expected);
    }
}
