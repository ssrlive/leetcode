#![allow(dead_code)]

/*

// 2299. Strong Password Checker II
// https://leetcode.com/problems/strong-password-checker-ii/
// https://leetcode.cn/problems/strong-password-checker-ii/
//
// Easy
//
// A password is said to be strong if it satisfies all the following criteria:

    It has at least 8 characters.
    It contains at least one lowercase letter.
    It contains at least one uppercase letter.
    It contains at least one digit.
    It contains at least one special character. The special characters are the characters in the following string: "!@#$%^&*()-+".
    It does not contain 2 of the same character in adjacent positions (i.e., "aab" violates this condition, but "aba" does not).

Given a string password, return true if it is a strong password. Otherwise, return false.

Example 1:

Input: password = "IloveLe3tcode!"
Output: true
Explanation: The password meets all the requirements. Therefore, we return true.

Example 2:

Input: password = "Me+You--IsMyDream"
Output: false
Explanation: The password does not contain a digit and also contains 2 of the same character in adjacent positions. Therefore, we return false.

Example 3:

Input: password = "1aB!"
Output: false
Explanation: The password does not meet the length requirement. Therefore, we return false.

Constraints:

    1 <= password.length <= 100
    password consists of letters, digits, and special characters: "!@#$%^&*()-+".
*/

struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let mut res = 0;
        let mut prev_char = ' ';
        for c in password.chars() {
            match c == prev_char {
                true => return false,
                _ if c.is_ascii_lowercase() => res |= 0b1,
                _ if c.is_ascii_uppercase() => res |= 0b10,
                _ if c.is_ascii_digit() => res |= 0b100,
                _ => res |= 0b1000, // "!@#$%^&*()-+"
            }
            prev_char = c;
        }
        res == 0b1111
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("IloveLe3tcode!", true),
        ("Me+You--IsMyDream", false),
        ("1aB!", false),
    ];
    for (p, e) in cases {
        assert_eq!(Solution::strong_password_checker_ii(p.to_string()), e);
    }
}
