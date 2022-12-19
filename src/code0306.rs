#![allow(dead_code)]

// 306. Additive Number
// https://leetcode.com/problems/additive-number/
// https://leetcode.cn/problems/additive-number/
//
// Additive number is a string whose digits can form additive sequence.
//
// A valid additive sequence should contain at least three numbers. Except for the first two numbers,
// each subsequent number in the sequence must be the sum of the preceding two.
//
// Given a string num containing only digits '0'-'9', return true if it's an additive number, or false otherwise.
//
// Note: Numbers in the additive sequence cannot have leading zeros, so sequence 1, 2, 03 or 1, 02, 3 is invalid.
//
// Example 1:
//
// Input: num = "112358"
// Output: true
// Explanation: The digits can form an additive sequence: 1, 1, 2, 3, 5, 8.
//              1 + 1 = 2
//              1 + 2 = 3
//              2 + 3 = 5
//              3 + 5 = 8
//
// Example 2:
//
// Input: num = "199100199"
// Output: true
// Explanation: The additive sequence is: 1, 99, 100, 199.
//              1 + 99 = 100
//              99 + 100 = 199
//
// Constraints:
//
// - 1 <= num.length <= 35
// - num consists of only digits '0'-'9'.
//
// Follow up: How would you handle overflow for very large input integers?
//

struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn _is_additive_number(num: String) -> Option<bool> {
            for i in 1..num.len() - 1 {
                if i > 1 && &num[0..1] == "0" {
                    continue;
                }
                for j in i + 1..num.len() {
                    if j - i > 1 && &num[i..i + 1] == "0" {
                        continue;
                    }
                    let mut n1 = num[0..i].parse::<u64>().ok()?;
                    let mut n2 = num[i..j].parse::<u64>().ok()?;
                    let mut s = num[0..j].to_string();
                    loop {
                        let n3 = n1 + n2;
                        n1 = n2;
                        n2 = n3;
                        s += n3.to_string().as_str();
                        if s == num {
                            return Some(true);
                        }
                        if !num.starts_with(&s) {
                            break;
                        }
                    }
                }
            }
            Some(false)
        }

        _is_additive_number(num).unwrap_or(false)
    }
}

#[test]
fn test_additive_number() {
    assert!(Solution::is_additive_number("112358".to_string()));
    assert!(Solution::is_additive_number("199100199".to_string()));
    assert!(!Solution::is_additive_number("199100190".to_string()));
    assert!(!Solution::is_additive_number("199100191".to_string()));
    assert!(!Solution::is_additive_number("199100192".to_string()));
    assert!(!Solution::is_additive_number("199100193".to_string()));
    assert!(!Solution::is_additive_number("199100194".to_string()));
    assert!(!Solution::is_additive_number("199100195".to_string()));
    assert!(!Solution::is_additive_number("199100196".to_string()));
    assert!(!Solution::is_additive_number("199100197".to_string()));
    assert!(!Solution::is_additive_number("199100198".to_string()));
    assert!(!Solution::is_additive_number("1991001990".to_string()));
    assert!(!Solution::is_additive_number("1991001991".to_string()));
    assert!(!Solution::is_additive_number("1991001992".to_string()));
    assert!(!Solution::is_additive_number("1991001993".to_string()));
    assert!(!Solution::is_additive_number("1991001994".to_string()));
    assert!(!Solution::is_additive_number("1991001995".to_string()));
    assert!(!Solution::is_additive_number("1991001996".to_string()));
    assert!(!Solution::is_additive_number("1991001997".to_string()));
    assert!(!Solution::is_additive_number("1991001998".to_string()));
}
