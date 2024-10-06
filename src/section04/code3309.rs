#![allow(dead_code)]

// 3309. Maximum Possible Number by Binary Concatenation
// https://leetcode.com/problems/maximum-possible-number-by-binary-concatenation/
// https://leetcode.cn/problems/maximum-possible-number-by-binary-concatenation/
//
// Medium
//
// You are given an array of integers nums of size 3.
//
// Return the maximum possible number whose binary representation can be formed by concatenating
// the binary representation of all elements in nums in some order.
//
// Note that the binary representation of any number does not contain leading zeros.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 30
//
// Explanation:
//
// Concatenate the numbers in the order [3, 1, 2] to get the result "11110", which is the binary representation of 30.
//
// Example 2:
//
// Input: nums = [2,8,16]
//
// Output: 1296
//
// Explanation:
//
// Concatenate the numbers in the order [2, 8, 16] to get the result "10100010000", which is the binary representation of 1296.
//
// Constraints:
//
//     nums.length == 3
//     1 <= nums[i] <= 127
//

struct Solution;

impl Solution {
    pub fn max_good_number(nums: Vec<i32>) -> i32 {
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        let permutations = vec![
            vec![a, b, c],
            vec![a, c, b],
            vec![b, a, c],
            vec![b, c, a],
            vec![c, a, b],
            vec![c, b, a],
        ];
        permutations
            .into_iter()
            .map(|v| {
                let binary_concat = format!("{:b}{:b}{:b}", v[0], v[1], v[2]);
                i32::from_str_radix(&binary_concat, 2).unwrap()
            })
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_good_number(vec![1, 2, 3]), 30);
    assert_eq!(Solution::max_good_number(vec![2, 8, 16]), 1296);
    assert_eq!(Solution::max_good_number(vec![8, 2, 16]), 1296);
}
