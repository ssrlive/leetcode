#![allow(dead_code)]

/*

// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/
// https://leetcode.cn/problems/running-sum-of-1d-array/
//
// Easy
//
// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
//
// Return the running sum of nums.
//
// Example 1:

Input: nums = [1,2,3,4]
Output: [1,3,6,10]
Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].

Example 2:

Input: nums = [1,1,1,1,1]
Output: [1,2,3,4,5]
Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].

Example 3:

Input: nums = [3,1,2,10,1]
Output: [3,4,6,16,17]

Constraints:

    1 <= nums.length <= 1000
    -10^6 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut res = vec![];
        for num in nums {
            sum += num;
            res.push(sum);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], vec![1, 3, 6, 10]),
        (vec![1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5]),
        (vec![3, 1, 2, 10, 1], vec![3, 4, 6, 16, 17]),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::running_sum(nums), expected);
    }
}
