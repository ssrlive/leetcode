#![allow(dead_code)]

/*

// 2652. Sum Multiples
// https://leetcode.com/problems/sum-multiples/
// https://leetcode.cn/problems/sum-multiples/
//
// Easy
//
// Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.

Return an integer denoting the sum of all numbers in the given range satisfying the constraint.

Example 1:

Input: n = 7
Output: 21
Explanation: Numbers in the range [1, 7] that are divisible by 3, 5, or 7 are 3, 5, 6, 7. The sum of these numbers is 21.

Example 2:

Input: n = 10
Output: 40
Explanation: Numbers in the range [1, 10] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9, 10. The sum of these numbers is 40.

Example 3:

Input: n = 9
Output: 30
Explanation: Numbers in the range [1, 9] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9. The sum of these numbers is 30.

Constraints:

    1 <= n <= 10^3
*/

struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (3..=n).filter(|&x| x % 3 == 0 || x % 5 == 0 || x % 7 == 0).sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_multiples(7), 21);
    assert_eq!(Solution::sum_of_multiples(10), 40);
    assert_eq!(Solution::sum_of_multiples(9), 30);
}
