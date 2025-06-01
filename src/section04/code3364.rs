#![allow(dead_code)]

// 3364. Minimum Positive Sum Subarray
// https://leetcode.com/problems/minimum-positive-sum-subarray/
// https://leetcode.cn/problems/minimum-positive-sum-subarray/
//
// Easy
//
// You are given an integer array nums and two integers l and r. Your task is to find the minimum sum of
// a subarray whose size is between l and r (inclusive) and whose sum is greater than 0.
//
// Return the minimum sum of such a subarray. If no such subarray exists, return -1.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [3, -2, 1, 4], l = 2, r = 3
//
// Output: 1
//
// Explanation:
//
// The subarrays of length between l = 2 and r = 3 where the sum is greater than 0 are:
//
// [3, -2] with a sum of 1
// [1, 4] with a sum of 5
// [3, -2, 1] with a sum of 2
// [-2, 1, 4] with a sum of 3
// Out of these, the subarray [3, -2] has a sum of 1, which is the smallest positive sum. Hence, the answer is 1.
//
// Example 2:
//
// Input: nums = [-2, 2, -3, 1], l = 2, r = 3
//
// Output: -1
//
// Explanation:
//
// There is no subarray of length between l and r that has a sum greater than 0. So, the answer is -1.
//
// Example 3:
//
// Input: nums = [1, 2, 3, 4], l = 2, r = 4
//
// Output: 3
//
// Explanation:
//
// The subarray [1, 2] has a length of 2 and the minimum sum greater than 0. So, the answer is 3.
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= l <= r <= nums.length
// -1000 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut min_sum: i32 = i32::MAX;

        for n in l..=r {
            let mut sum = 0;

            for j in 0..n {
                sum += nums[j as usize];
            }

            if min_sum > sum && sum > 0 {
                min_sum = sum;
            }
            println!("{n}");

            for i in n..nums.len() as i32 {
                sum += nums[i as usize];
                sum -= nums[i as usize - n as usize];
                println!("{sum}");

                if min_sum > sum && sum > 0 {
                    min_sum = sum;
                }
            }
        }

        if min_sum == i32::MAX {
            return -1;
        }

        min_sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
    assert_eq!(Solution::minimum_sum_subarray(vec![-2, 2, -3, 1], 2, 3), -1);
    assert_eq!(Solution::minimum_sum_subarray(vec![1, 2, 3, 4], 2, 4), 3);
}
