#![allow(dead_code)]

// 1018. Binary Prefix Divisible By 5
// https://leetcode.com/problems/binary-prefix-divisible-by-5/
// https://leetcode.cn/problems/binary-prefix-divisible-by-5/
//
// You are given a binary array nums (0-indexed).
//
// We define xi as the number whose binary representation is the subarray nums[0..i] (from most-significant-bit to least-significant-bit).
//
// For example, if nums = [1,0,1], then x0 = 1, x1 = 2, and x2 = 5.
// Return an array of booleans answer where answer[i] is true if xi is divisible by 5.
//
// Example 1:
//
// Input: nums = [0,1,1]
// Output: [true,false,false]
// Explanation: The input numbers in binary are 0, 01, 011; which are 0, 1, and 3 in base-10.
// Only the first number is divisible by 5, so answer[0] is true.
//
// Example 2:
//
// Input: nums = [1,1,1]
// Output: [false,false,false]
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - nums[i] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut res = Vec::with_capacity(nums.len());
        let mut val = 0;

        for &num in &nums {
            val = (val << 1 | num) % 5;
            res.push(val == 0);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![0, 1, 1], vec![true, false, false]),
        (vec![1, 1, 1], vec![false, false, false]),
        (vec![0, 1, 1, 1, 1, 1], vec![true, false, false, false, true, false]),
        (vec![1, 1, 1, 0, 1], vec![false, false, false, false, false]),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::prefixes_div_by5(nums), expected);
    }
}
