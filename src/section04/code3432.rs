#![allow(dead_code)]

// 3432. Count Partitions with Even Sum Difference
// https://leetcode.com/problems/count-partitions-with-even-sum-difference/
// https://leetcode.cn/problems/count-partitions-with-even-sum-difference/
//
// Easy
//
// You are given an integer array nums of length n.
//
// A partition is defined as an index i where 0 <= i < n - 1, splitting the array into two non-empty subarrays such that:
//
//     Left subarray contains indices [0, i].
//     Right subarray contains indices [i + 1, n - 1].
//
// Return the number of partitions where the difference between the sum of the left and right subarrays is even.
//
// Example 1:
//
// Input: nums = [10,10,3,7,6]
//
// Output: 4
//
// Explanation:
//
// The 4 partitions are:
//
//     [10], [10, 3, 7, 6] with a sum difference of 10 - 26 = -16, which is even.
//     [10, 10], [3, 7, 6] with a sum difference of 20 - 16 = 4, which is even.
//     [10, 10, 3], [7, 6] with a sum difference of 23 - 13 = 10, which is even.
//     [10, 10, 3, 7], [6] with a sum difference of 30 - 6 = 24, which is even.
//
// Example 2:
//
// Input: nums = [1,2,2]
//
// Output: 0
//
// Explanation:
//
// No partition results in an even sum difference.
//
// Example 3:
//
// Input: nums = [2,4,6,8]
//
// Output: 3
//
// Explanation:
//
// All partitions result in an even sum difference.
//
// Constraints:
//
//     2 <= n == nums.length <= 100
//     1 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() % 2 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_partitions(vec![10, 10, 3, 7, 6]), 4);
    assert_eq!(Solution::count_partitions(vec![1, 2, 2]), 0);
    assert_eq!(Solution::count_partitions(vec![2, 4, 6, 8]), 3);
}
