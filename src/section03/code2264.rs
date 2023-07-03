#![allow(dead_code)]

/*

// 2264. Largest 3-Same-Digit Number in String
// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// https://leetcode.cn/problems/largest-3-same-digit-number-in-string/
//
// Easy
//
// You are given a string num representing a large integer. An integer is good if it meets the following conditions:

    It is a substring of num with length 3.
    It consists of only one unique digit.

Return the maximum good integer as a string or an empty string "" if no such integer exists.

Note:

    A substring is a contiguous sequence of characters within a string.
    There may be leading zeroes in num or a good integer.

Example 1:

Input: num = "6777133339"
Output: "777"
Explanation: There are two distinct good integers: "777" and "333".
"777" is the largest, so we return "777".

Example 2:

Input: num = "2300019"
Output: "000"
Explanation: "000" is the only good integer.

Example 3:

Input: num = "42352338"
Output: ""
Explanation: No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.

Constraints:

    3 <= num.length <= 1000
    num only consists of digits.
*/

struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.chars()
            .zip(num.chars().skip(1))
            .zip(num.chars().skip(2))
            .filter_map(|((a, b), c)| if a == b && b == c { Some(format!("{}{}{}", a, b, c)) } else { None })
            .max()
            .unwrap_or_default()
    }
}

#[test]
fn test() {
    let cases = vec![("6777133339", "777"), ("2300019", "000"), ("42352338", "")];
    for (num, expect) in cases {
        assert_eq!(Solution::largest_good_integer(num.to_string()), expect);
    }
}
