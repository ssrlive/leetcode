#![allow(dead_code)]

// 1108. Defanging an IP Address
// https://leetcode.com/problems/defanging-an-ip-address/
// https://leetcode.cn/problems/defanging-an-ip-address/
//
// Given a valid (IPv4) IP address, return a defanged version of that IP address.
//
// A defanged IP address replaces every period "." with "[.]".
//
// Example 1:
//
// Input: address = "1.1.1.1"
// Output: "1[.]1[.]1[.]1"
//
// Example 2:
//
// Input: address = "255.100.50.0"
// Output: "255[.]100[.]50[.]0"
//
// Constraints:
//
// - The given address is a valid IPv4 address.
//

struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::from("");
        for c in address.chars() {
            match c {
                '.' => result.push_str("[.]"),
                _ => result.push(c),
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![("1.1.1.1", "1[.]1[.]1[.]1"), ("255.100.50.0", "255[.]100[.]50[.]0")];
    for (address, expected) in cases {
        assert_eq!(Solution::defang_i_paddr(address.to_string()), expected);
    }
}
