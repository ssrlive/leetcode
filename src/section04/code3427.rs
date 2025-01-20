#![allow(dead_code)]

// 3427. Sum of Variable Length Subarrays
// https://leetcode.com/problems/sum-of-variable-length-subarrays/
// https://leetcode.cn/problems/sum-of-variable-length-subarrays/
//
// Easy
//
// You are given an integer array nums of size n. For each index i where 0 <= i < n, define a subarray
// nums[start ... i] where start = max(0, i - nums[i]).
//
// Return the total sum of all elements from the subarray defined for each index in the array.
//
// Example 1:
//
// Input: nums = [2,3,1]
//
// Output: 11
//
// Explanation:
// i	Subarray	Sum
// 0	nums[0] = [2]	2
// 1	nums[0 ... 1] = [2, 3]	5
// 2	nums[1 ... 2] = [3, 1]	4
// Total Sum	 	11
//
// The total sum is 11. Hence, 11 is the output.
//
// Example 2:
//
// Input: nums = [3,1,1,2]
//
// Output: 13
//
// Explanation:
// i	Subarray	Sum
// 0	nums[0] = [3]	3
// 1	nums[0 ... 1] = [3, 1]	4
// 2	nums[1 ... 2] = [1, 1]	2
// 3	nums[1 ... 3] = [1, 1, 2]	4
// Total Sum	 	13
//
// The total sum is 13. Hence, 13 is the output.
//
// Constraints:
//
//     1 <= n == nums.length <= 100
//     1 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut diff = vec![0; n + 1];
        for i in 0..n {
            let start = 0.max(i as i32 - nums[i]);
            diff[start as usize] += 1;
            diff[i + 1] -= 1;
        }
        for i in 1..n {
            diff[i] += diff[i - 1];
        }
        let mut sub_sum = 0;
        for i in 0..n {
            sub_sum += nums[i] * diff[i];
        }
        sub_sum
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1];
    let res = 11;
    assert_eq!(Solution::subarray_sum(nums), res);

    let nums = vec![3, 1, 1, 2];
    let res = 13;
    assert_eq!(Solution::subarray_sum(nums), res);
}
