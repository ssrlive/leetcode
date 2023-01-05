#![allow(dead_code)]

// 970. Powerful Integers
// https://leetcode.com/problems/powerful-integers/
// https://leetcode.cn/problems/powerful-integers/
//
// Given three integers x, y, and bound, return a list of all the powerful integers that have a value less than or equal to bound.
//
// An integer is powerful if it can be represented as xi + yj for some integers i >= 0 and j >= 0.
//
// You may return the answer in any order. In your answer, each value should occur at most once.
//
// Example 1:
//
// Input: x = 2, y = 3, bound = 10
// Output: [2,3,4,5,7,9,10]
// Explanation:
// 2 = 20 + 30
// 3 = 21 + 30
// 4 = 20 + 31
// 5 = 21 + 31
// 7 = 22 + 31
// 9 = 23 + 30
// 10 = 20 + 32
//
// Example 2:
//
// Input: x = 3, y = 5, bound = 15
// Output: [2,4,6,8,10,14]
//
// Constraints:
//
// - 1 <= x, y <= 100
// - 0 <= bound <= 10^6
//

struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;
        use std::iter::successors;

        successors(Some(1), |n| Some(n * x).filter(|&xi| x > 1 && xi < bound))
            .flat_map(|xi| {
                successors(Some(1), move |n| Some(n * y).filter(|&yj| y > 1 && yj <= bound - xi))
                    .filter_map(move |yj| Some(xi + yj).filter(|&sum| sum <= bound))
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

#[test]
fn test() {
    let mut result = Solution::powerful_integers(2, 3, 10);
    result.sort();
    assert_eq!(result, vec![2, 3, 4, 5, 7, 9, 10]);

    let mut result = Solution::powerful_integers(3, 5, 15);
    result.sort();
    assert_eq!(result, vec![2, 4, 6, 8, 10, 14]);
}
