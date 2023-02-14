#![allow(dead_code)]

/*

// 1608. Special Array With X Elements Greater Than or Equal X
// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/
// https://leetcode.cn/problems/special-array-with-x-elements-greater-than-or-equal-x/
//
// Easy
//
// You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.

Notice that x does not have to be an element in nums.

Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.

Example 1:

Input: nums = [3,5]
Output: 2
Explanation: There are 2 values (3 and 5) that are greater than or equal to 2.

Example 2:

Input: nums = [0,0]
Output: -1
Explanation: No numbers fit the criteria for x.
If x = 0, there should be 0 numbers >= x, but there are 2.
If x = 1, there should be 1 number >= x, but there are 0.
If x = 2, there should be 2 numbers >= x, but there are 0.
x cannot be greater since there are only 2 numbers in nums.

Example 3:

Input: nums = [0,4,3,0,4]
Output: 3
Explanation: There are 3 values that are greater than or equal to 3.

Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let arr_count = nums.iter().fold([0; 1001], |mut acc, &x| {
            acc[x as usize] += 1;
            acc
        });
        let mut prev_sum = 0;
        let l = nums.len() as i32;

        for i in 0..=l {
            let after_sum = l - prev_sum;
            prev_sum += arr_count[i as usize];

            if after_sum == i {
                return i;
            } else if prev_sum == l {
                break;
            }
        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::special_array(vec![3, 5]), 2);
    assert_eq!(Solution::special_array(vec![0, 0]), -1);
    assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
}
