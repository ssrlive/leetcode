#![allow(dead_code)]

/*

// 1979. Find Greatest Common Divisor of Array
// https://leetcode.com/problems/find-greatest-common-divisor-of-array/
// https://leetcode.cn/problems/find-greatest-common-divisor-of-array/
//
// Easy
//
// Given an integer array nums, return the greatest common divisor of the smallest number and largest number in nums.

The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.

Example 1:

Input: nums = [2,5,6,9,10]
Output: 2
Explanation:
The smallest number in nums is 2.
The largest number in nums is 10.
The greatest common divisor of 2 and 10 is 2.

Example 2:

Input: nums = [7,5,6,8,3]
Output: 1
Explanation:
The smallest number in nums is 3.
The largest number in nums is 8.
The greatest common divisor of 3 and 8 is 1.

Example 3:

Input: nums = [3,3]
Output: 3
Explanation:
The smallest number in nums is 3.
The largest number in nums is 3.
The greatest common divisor of 3 and 3 is 3.

Constraints:

    2 <= nums.length <= 1000
    1 <= nums[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            match a == 0 {
                true => b,
                false => gcd(b % a, a),
            }
        }

        let (min_val, max_val) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_val, max_val), &x| (min_val.min(x), max_val.max(x)));

        gcd(max_val, min_val)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
    assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
    assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
}
