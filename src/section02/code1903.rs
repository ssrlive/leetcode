#![allow(dead_code)]

/*

// 1903. Largest Odd Number in String
// https://leetcode.com/problems/largest-odd-number-in-string/
// https://leetcode.cn/problems/largest-odd-number-in-string/
//
// Easy
//
// You are given a string num, representing a large integer. Return the largest-valued odd integer (as a string) that is a non-empty substring of num, or an empty string "" if no odd integer exists.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: num = "52"
Output: "5"
Explanation: The only non-empty substrings are "5", "2", and "52". "5" is the only odd number.

Example 2:

Input: num = "4206"
Output: ""
Explanation: There are no odd numbers in "4206".

Example 3:

Input: num = "35427"
Output: "35427"
Explanation: "35427" is already an odd number.

Constraints:

    1 <= num.length <= 10^5
    num only consists of digits and does not contain any leading zeros.
*/

struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let max_odd_ind = num.char_indices().filter(|&(_, c)| (c as u8) % 2 == 1).last();
        match max_odd_ind {
            None => "",
            Some((ind, _)) => &num[..=ind],
        }
        .to_string()
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("52", "5"),
        ("4206", ""),
        ("35427", "35427"),
    ];
    for (num, expect) in cases {
        assert_eq!(Solution::largest_odd_number(num.to_string()), expect);
    }
}
