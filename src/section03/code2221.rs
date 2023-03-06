#![allow(dead_code)]

/*

// 2221. Find Triangular Sum of an Array
// https://leetcode.com/problems/find-triangular-sum-of-an-array/
// https://leetcode.cn/problems/find-triangular-sum-of-an-array/
//
// Medium
//
// You are given a 0-indexed integer array nums, where nums[i] is a digit between 0 and 9 (inclusive).

The triangular sum of nums is the value of the only element present in nums after the following process terminates:

    Let nums comprise of n elements. If n == 1, end the process. Otherwise, create a new 0-indexed integer array newNums of length n - 1.
    For each index i, where 0 <= i < n - 1, assign the value of newNums[i] as (nums[i] + nums[i+1]) % 10, where % denotes modulo operator.
    Replace the array nums with newNums.
    Repeat the entire process starting from step 1.

Return the triangular sum of nums.

Example 1:

Input: nums = [1,2,3,4,5]
Output: 8
Explanation:
The above diagram depicts the process from which we obtain the triangular sum of the array.

Example 2:

Input: nums = [5]
Output: 5
Explanation:
Since there is only one element in nums, the triangular sum is the value of that element itself.

Constraints:

    1 <= nums.length <= 1000
    0 <= nums[i] <= 9
*/

struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while nums.len() > 1 {
            let mut new_nums = vec![];
            for i in 0..nums.len() - 1 {
                new_nums.push((nums[i] + nums[i + 1]) % 10);
            }
            nums = new_nums;
        }
        nums[0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    assert_eq!(Solution::triangular_sum(vec![5]), 5);
}
