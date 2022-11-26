#![allow(dead_code)]

// 420. Strong Password Checker
// https://leetcode.com/problems/strong-password-checker/
//
// A password is considered strong if below conditions are all met:
//
// - It has at least 6 characters and at most 20 characters.
// - It must contain at least one lowercase letter, at least one uppercase letter, and at least one digit.
// - It must NOT contain three repeating characters in a row ("...aaa..." is weak, but "...aa...a..." is strong, assuming other conditions are met).
//
// Given a string password, return the minimum number of steps required to make password strong. if password is already strong, return 0.
//
// In one step, you can:
//
// - Insert one character to password,
// - Delete one character from password, or
// - Replace one character of password with another character.
//
// Example 1:
//
// Input: password = "a"
// Output: 5
//
// Example 2:
//
// Input: password = "aA1"
// Output: 3
//
// Example 3:
//
// Input: password = "1337C0d3"
// Output: 0
//
// Constraints:
//
// - 1 <= password.length <= 50
// - password consists of letters, digits, dot '.' or exclamation mark '!'.
//

struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        pub fn _strong_password_checker(password: String) -> Option<i64> {
            let mut mislchar = true;
            let mut misdigit = true;
            let mut misuchar = true;
            for c in password.chars() {
                if c.is_ascii_digit() {
                    misdigit = false;
                } else if c.is_lowercase() {
                    mislchar = false;
                } else if c.is_uppercase() {
                    misuchar = false;
                }
            }
            let missnum = mislchar as i64 + misuchar as i64 + misdigit as i64;
            let mut done = 0;
            let mut dtwo = 0;
            let mut replace = 0;
            let mut i = 2;
            while i < password.len() {
                if password.chars().nth(i)? == password.chars().nth(i - 1)?
                    && password.chars().nth(i - 1)? == password.chars().nth(i - 2)?
                {
                    let mut len = 3;
                    while i + 1 < password.len() && password.chars().nth(i + 1)? == password.chars().nth(i)? {
                        i += 1;
                        len += 1;
                    }
                    if len % 3 == 0 {
                        done += 1;
                    } else if len % 3 == 1 {
                        dtwo += 1;
                    }
                    replace += len / 3;
                }
                i += 1;
            }
            let result;
            if password.len() < 6 {
                result = std::cmp::max(6 - password.len(), missnum as usize) as i64;
            } else if password.len() <= 20 {
                result = std::cmp::max(replace, missnum);
            } else {
                let del = (password.len() - 20) as i64;
                replace -= std::cmp::min(del, done);
                if del - done > 0 {
                    replace -= std::cmp::min((del - done) / 2, dtwo);
                }
                if del - done - 2 * dtwo > 0 {
                    replace -= (del - done - 2 * dtwo) / 3;
                }
                result = del + std::cmp::max(replace, missnum);
            }
            Some(result)
        }
        _strong_password_checker(password).unwrap_or_default() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::strong_password_checker("a".to_string()), 5);
    assert_eq!(Solution::strong_password_checker("aA1".to_string()), 3);
    assert_eq!(Solution::strong_password_checker("1337C0d3".to_string()), 0);
    assert_eq!(
        Solution::strong_password_checker("aaaaAAAAAA000000123456".to_string()),
        5
    );
}
