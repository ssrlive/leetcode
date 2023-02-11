#![allow(dead_code)]

/*

// 1567. Maximum Length of Subarray With Positive Product
// https://leetcode.com/problems/maximum-length-of-subarray-with-positive-product/
// https://leetcode.cn/problems/maximum-length-of-subarray-with-positive-product/
//
// Medium
//
// Given an array of integers nums, find the maximum length of a subarray where the product of all its elements is positive.

A subarray of an array is a consecutive sequence of zero or more values taken out of that array.

Return the maximum length of a subarray with positive product.

Example 1:

Input: nums = [1,-2,-3,4]
Output: 4
Explanation: The array nums already has a positive product of 24.

Example 2:

Input: nums = [0,1,-2,-3,-4]
Output: 3
Explanation: The longest subarray with positive product is [1,-2,-3] which has a product of 6.
Notice that we cannot include 0 in the subarray since that'll make the product 0 which is not positive.

Example 3:

Input: nums = [-1,-2,-3,0,1]
Output: 2
Explanation: The longest subarray with positive product is [-1,-2] or [-2,-3].

Constraints:

    1 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut pos = 0;
        let mut neg = 0;
        for n in nums {
            match n.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    pos = 0;
                    neg = 0;
                }
                std::cmp::Ordering::Greater => {
                    pos += 1;
                    if neg > 0 {
                        neg += 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    let tmp = pos;
                    if neg > 0 {
                        pos = neg + 1;
                    } else {
                        pos = 0;
                    }
                    neg = tmp + 1;
                }
            }
            max = max.max(pos);
        }
        max
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, -2, -3, 4], 4),
        (vec![0, 1, -2, -3, -4], 3),
        (vec![-1, -2, -3, 0, 1], 2),
        (vec![1, 2, 3, 5, -6, 4, 0, 10], 4),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::get_max_len(nums), expected);
    }
}
