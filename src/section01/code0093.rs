#![allow(dead_code)]

// 93. Restore IP Addresses
// https://leetcode.com/problems/restore-ip-addresses/
// https://leetcode.cn/problems/restore-ip-addresses/
//
// A valid IP address consists of exactly four integers separated by single dots. Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.
//
// For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
// Given a string s containing only digits, return all possible valid IP addresses that can be formed by inserting dots into s. You are not allowed to reorder or remove any digits in s. You may return the valid IP addresses in any order.
//
// Example 1:
//
// Input: s = "25525511135"
// Output: ["255.255.11.135","255.255.111.35"]
//
// Example 2:
//
// Input: s = "0000"
// Output: ["0.0.0.0"]
//
// Example 3:
//
// Input: s = "101023"
// Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
//
// Constraints:
//
// - 1 <= s.length <= 20
// - s consists of digits only.
//

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn is_valid(s: &str) -> bool {
            match s.len() {
                1 => true,
                2 => s[0..1] != *"0",
                3 => s[0..1] != *"0" && s[0..=2] <= *"255",
                _ => unreachable!(),
            }
        }

        fn backtrace<'a>(parts: &mut Vec<&'a str>, str: &'a str, start: usize, results: &mut Vec<String>) {
            if parts.capacity() == parts.len() {
                if start == str.len() {
                    let ip = parts.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(".");
                    results.push(ip);
                }
                return;
            }

            let lengths = (1..=std::cmp::min(3, str.len() - start)).filter(|len| is_valid(&str[start..start + len]));
            for len in lengths {
                parts.push(&str[start..start + len]);
                backtrace(parts, str, start + len, results);
                parts.pop();
            }
        }

        let mut results = Vec::new();
        let mut buffer = Vec::with_capacity(4);
        backtrace(&mut buffer, &s, 0, &mut results);

        results
    }
}

#[test]
fn test() {
    let cases = vec![
        ("25525511135", vec!["255.255.11.135", "255.255.111.35"]),
        ("0000", vec!["0.0.0.0"]),
        (
            "101023",
            vec!["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"],
        ),
    ];
    for (s, expected) in cases {
        let result = Solution::restore_ip_addresses(s.to_string());
        assert_eq!(result, expected);
    }
}
