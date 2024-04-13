#![allow(dead_code)]

// 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
// https://leetcode.cn/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
//
// Easy
//
// You are given an array of integers nums. Return the length of the longest subarray of nums
// which is either strictly increasing or strictly decreasing.
//
// Example 1:
//
// Input: nums = [1,4,3,3,2]
//
// Output: 2
//
// Explanation:
//
// The strictly increasing subarrays of nums are [1], [2], [3], [3], [4], and [1,4].
//
// The strictly decreasing subarrays of nums are [1], [2], [3], [3], [4], [3,2], and [4,3].
//
// Hence, we return 2.
//
// Example 2:
//
// Input: nums = [3,3,3,3]
//
// Output: 1
//
// Explanation:
//
// The strictly increasing subarrays of nums are [3], [3], [3], and [3].
//
// The strictly decreasing subarrays of nums are [3], [3], [3], and [3].
//
// Hence, we return 1.
//
// Example 3:
//
// Input: nums = [3,2,1]
//
// Output: 3
//
// Explanation:
//
// The strictly increasing subarrays of nums are [3], [2], and [1].
//
// The strictly decreasing subarrays of nums are [3], [2], [1], [3,2], [2,1], and [3,2,1].
//
// Hence, we return 3.
//
// Constraints:
//
//     1 <= nums.length <= 50
//     1 <= nums[i] <= 50
//

struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut prev, mut trend, mut count, mut result) = (*nums.first().unwrap(), 0, 0, 0);

        for &num in nums.iter().skip(1) {
            match num.cmp(&prev) {
                std::cmp::Ordering::Greater => {
                    if trend == 1 {
                        count += 1;
                    } else {
                        trend = 1;
                        count = 1;
                    }
                }
                std::cmp::Ordering::Less => {
                    if trend == -1 {
                        count += 1;
                    } else {
                        trend = -1;
                        count = 1;
                    }
                }
                std::cmp::Ordering::Equal => {
                    trend = 0;
                    count = 0;
                }
            }
            result = result.max(count);
            prev = num;
        }
        result + 1
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 3, 3, 2];
    assert_eq!(Solution::longest_monotonic_subarray(nums), 2);

    let nums = vec![3, 3, 3, 3];
    assert_eq!(Solution::longest_monotonic_subarray(nums), 1);

    let nums = vec![3, 2, 1];
    assert_eq!(Solution::longest_monotonic_subarray(nums), 3);
}
