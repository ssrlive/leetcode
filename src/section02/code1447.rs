#![allow(dead_code)]

// 1447. Simplified Fractions
// https://leetcode.com/problems/simplified-fractions/
// https://leetcode.cn/problems/simplified-fractions/
//
// Medium
//
// Given an integer n, return a list of all simplified fractions between 0 and 1 (exclusive) such that the
// denominator is less-than-or-equal-to n. You can return the answer in any order.
//
// Example 1:
//
// Input: n = 2
// Output: ["1/2"]
// Explanation: "1/2" is the only unique fraction with a denominator less-than-or-equal-to 2.
//
// Example 2:
//
// Input: n = 3
// Output: ["1/2","1/3","2/3"]
//
// Example 3:
//
// Input: n = 4
// Output: ["1/2","1/3","1/4","2/3","3/4"]
// Explanation: "2/4" is not a simplified fraction because it can be simplified to "1/2".
//
// Constraints:
//
// -    1 <= n <= 100
//

struct Solution;

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        let mut res = vec![];
        for i in 2..=n {
            for j in 1..i {
                if gcd(i, j) == 1 {
                    res.push(format!("{j}/{i}"));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec!["1/2"]),
        (3, vec!["1/2", "1/3", "2/3"]),
        (4, vec!["1/2", "1/3", "2/3", "1/4", "3/4"]),
    ];
    for (n, expect) in cases {
        assert_eq!(Solution::simplified_fractions(n), expect);
    }
}
