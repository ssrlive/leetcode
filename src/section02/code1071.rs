#![allow(dead_code)]

// 1071. Greatest Common Divisor of Strings
// https://leetcode.com/problems/greatest-common-divisor-of-strings/
// https://leetcode.cn/problems/greatest-common-divisor-of-strings/
//
// For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
//
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
//
// Example 1:
//
// Input: str1 = "ABCABC", str2 = "ABC"
// Output: "ABC"
//
// Example 2:
//
// Input: str1 = "ABABAB", str2 = "ABAB"
// Output: "AB"
//
// Example 3:
//
// Input: str1 = "LEET", str2 = "CODE"
// Output: ""
//
// Constraints:
//
// - 1 <= str1.length, str2.length <= 1000
// - str1 and str2 consist of English uppercase letters.
//

struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn gcd(a: usize, b: usize) -> usize {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }
        let (sb1, sb2) = (str1.as_bytes(), str2.as_bytes());
        let (l1, l2) = (str1.len(), str2.len());
        let l = gcd(l1, l2);

        if sb1.iter().enumerate().all(|(i, &b1)| b1 == sb2[i % l]) && sb2.iter().enumerate().all(|(i, &b2)| b2 == sb1[i % l]) {
            String::from_utf8(sb1[..l].to_vec()).unwrap()
        } else {
            "".to_string()
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("ABCABC", "ABC", "ABC"),
        ("ABABAB", "ABAB", "AB"),
        ("LEET", "CODE", ""),
        (
            "TAUXXTAUXXTAUXXTAUXXTAUXX",
            "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX",
            "TAUXX",
        ),
    ];
    for (str1, str2, expected) in cases {
        assert_eq!(Solution::gcd_of_strings(str1.to_string(), str2.to_string()), expected);
    }
}
