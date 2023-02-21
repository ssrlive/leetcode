#![allow(dead_code)]

/*

// 1780. Check if Number is a Sum of Powers of Three
Medium
809
28
Companies

Given an integer n, return true if it is possible to represent n as the sum of distinct powers of three. Otherwise, return false.

An integer y is a power of three if there exists an integer x such that y == 3x.

Example 1:

Input: n = 12
Output: true
Explanation: 12 = 31 + 32

Example 2:

Input: n = 91
Output: true
Explanation: 91 = 30 + 32 + 34

Example 3:

Input: n = 21
Output: false

Constraints:

    1 <= n <= 10^7
*/

struct Solution;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut temp = n;
        while temp > 0 {
            if temp % 3 == 2 {
                return false;
            }
            temp /= 3;
        }
        true
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (12, true),
        (91, true),
        (21, false),
    ];
    for (n, expected) in cases {
        assert_eq!(Solution::check_powers_of_three(n), expected);
    }
}
