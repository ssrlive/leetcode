#![allow(dead_code)]

// 2427. Number of Common Factors
// https://leetcode.com/problems/number-of-common-factors/
// https://leetcode.cn/problems/number-of-common-factors/
//
// Given two positive integers a and b, return the number of common factors of a and b.
//
// An integer x is a common factor of a and b if x divides both a and b.
//
// Example 1:
//
// Input: a = 12, b = 6
// Output: 4
// Explanation: The common factors of 12 and 6 are 1, 2, 3, 6.
//
// Example 2:
//
// Input: a = 25, b = 30
// Output: 2
// Explanation: The common factors of 25 and 30 are 1, 5.
//
// Constraints:
//
// - 1 <= a, b <= 1000
//

struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        use std::collections::HashSet;

        fn get_factors(n: i32) -> HashSet<i32> {
            let it = match n % 2 == 0 {
                true => (1..=n / 2).step_by(1),
                false => (1..=n / 3).step_by(2),
            };
            it.filter(|&x| n % x == 0).collect()
        }

        if a == 1 || b == 1 {
            1
        } else if a == b {
            get_factors(a).len() as i32 + 1
        } else {
            let s1 = get_factors(a);
            let s2 = get_factors(b);
            let (min, max) = match a < b {
                true => (a, b),
                false => (b, a),
            };
            s1.intersection(&s2).count() as i32 + (max % min == 0) as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::common_factors(12, 6), 4);
    assert_eq!(Solution::common_factors(25, 30), 2);
    assert_eq!(Solution::common_factors(1, 1), 1);
    assert_eq!(Solution::common_factors(1, 2), 1);
}
