#![allow(dead_code)]

/*

// 1486. XOR Operation in an Array
// https://leetcode.com/problems/xor-operation-in-an-array/
// https://leetcode.cn/problems/xor-operation-in-an-array/
//
// Easy
//
// You are given an integer n and an integer start.

Define an array nums where nums[i] = start + 2 * i (0-indexed) and n == nums.length.

Return the bitwise XOR of all elements of nums.

Example 1:

Input: n = 5, start = 0
Output: 8
Explanation: Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
Where "^" corresponds to bitwise XOR operator.

Example 2:

Input: n = 4, start = 3
Output: 8
Explanation: Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.

Constraints:

    1 <= n <= 1000
    0 <= start <= 1000
    n == nums.length
*/

struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut ans = 0;
        for i in 0..n {
            ans ^= start + 2 * i;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::xor_operation(5, 0), 8);
    assert_eq!(Solution::xor_operation(4, 3), 8);
}
