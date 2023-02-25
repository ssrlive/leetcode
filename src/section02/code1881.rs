#![allow(dead_code)]

/*

// 1881. Maximum Value after Insertion
// https://leetcode.com/problems/maximum-value-after-insertion/
// https://leetcode.cn/problems/maximum-value-after-insertion/
//
// Medium
//
// You are given a very large integer n, represented as a string,​​​​​​ and an integer digit x. The digits in n and the digit x are in the inclusive range [1, 9], and n may represent a negative number.

You want to maximize n's numerical value by inserting x anywhere in the decimal representation of n​​​​​​. You cannot insert x to the left of the negative sign.

    For example, if n = 73 and x = 6, it would be best to insert it between 7 and 3, making n = 763.
    If n = -55 and x = 2, it would be best to insert it before the first 5, making n = -255.

Return a string representing the maximum value of n​​​​​​ after the insertion.

Example 1:

Input: n = "99", x = 9
Output: "999"
Explanation: The result is the same regardless of where you insert 9.

Example 2:

Input: n = "-13", x = 2
Output: "-123"
Explanation: You can make n one of {-213, -123, -132}, and the largest of those three is -123.

Constraints:

    1 <= n.length <= 10^5
    1 <= x <= 9
    The digits in n​​​ are in the range [1, 9].
    n is a valid representation of an integer.
    In the case of a negative n,​​​​​​ it will begin with '-'.
*/

struct Solution;

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut chars = n.as_bytes().to_vec();
        let c = char::from_digit(x as u32, 10).unwrap();
        let x = c as u8;

        if chars[0] == b'-' {
            for i in 1..chars.len() {
                if chars[i] > x {
                    chars.insert(i, x);
                    return String::from_utf8(chars).unwrap();
                }
            }
            chars.push(c as u8);
            String::from_utf8(chars).unwrap()
        } else {
            for i in 0..chars.len() {
                if chars[i] < x {
                    chars.insert(i, x);
                    return String::from_utf8(chars).unwrap();
                }
            }
            chars.push(c as u8);
            String::from_utf8(chars).unwrap()
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("99", 9, "999"),
        ("-13", 2, "-123"),
        (
            "469975787943862651173569913153377",
            3,
            "4699757879438632651173569913153377",
        ),
    ];
    for (n, x, expected) in cases {
        assert_eq!(Solution::max_value(n.to_string(), x), expected);
    }
}
