#![allow(dead_code)]

/*
// 1464. Maximum Product of Two Elements in an Array
Easy

Given the array of integers nums, you will choose two different indices i and j of that array. Return the maximum value of (nums[i]-1)*(nums[j]-1).

Example 1:

Input: nums = [3,4,5,2]
Output: 12
Explanation: If you choose the indices i=1 and j=2 (indexed from 0), you will get the maximum value, that is, (nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12.

Example 2:

Input: nums = [1,5,4,5]
Output: 16
Explanation: Choosing the indices i=1 and j=3 (indexed from 0), you will get the maximum value of (5-1)*(5-1) = 16.

Example 3:

Input: nums = [3,7]
Output: 12

Constraints:

    2 <= nums.length <= 500
    1 <= nums[i] <= 10^3
*/

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;
        for &num in nums.iter() {
            if num > max1 {
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max2 = num;
            }
        }
        (max1 - 1) * (max2 - 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(Solution::max_product(vec![3, 7]), 12);
}
