#![allow(dead_code)]

// 1317. Convert Integer to the Sum of Two No-Zero Integers
// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
// https://leetcode.cn/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
//
// Easy
//
// No-Zero integer is a positive integer that does not contain any 0 in its decimal representation.
//
// Given an integer n, return a list of two integers [a, b] where:
//
//     a and b are No-Zero integers.
//     a + b = n
//
// The test cases are generated so that there is at least one valid solution.
// If there are many valid solutions, you can return any of them.
//
// Example 1:
//
// Input: n = 2
// Output: [1,1]
// Explanation: Let a = 1 and b = 1.
// Both a and b are no-zero integers, and a + b = 2 = n.
//
// Example 2:
//
// Input: n = 11
// Output: [2,9]
// Explanation: Let a = 2 and b = 9.
// Both a and b are no-zero integers, and a + b = 9 = n.
// Note that there are other valid answers as [8, 3] that can be accepted.
//
// Constraints:
//
// -    2 <= n <= 10^4
//

struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut a = 1;
        let mut b = n - 1;
        while a.to_string().contains('0') || b.to_string().contains('0') {
            a += 1;
            b -= 1;
        }
        vec![a, b]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
    assert_eq!(Solution::get_no_zero_integers(10000), vec![1, 9999]);
    assert_eq!(Solution::get_no_zero_integers(69), vec![1, 68]);
    assert_eq!(Solution::get_no_zero_integers(1010), vec![11, 999]);
}
