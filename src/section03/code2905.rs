#![allow(dead_code)]

// 2905. Find Indices With Index and Value Difference II
// https://leetcode.com/problems/find-indices-with-index-and-value-difference-ii/
// https://leetcode.cn/problems/find-indices-with-index-and-value-difference-ii/
//
// Medium
//
// You are given a 0-indexed integer array nums having length n, an integer indexDifference, and an integer valueDifference.
//
// Your task is to find two indices i and j, both in the range [0, n - 1], that satisfy the following conditions:
//
// abs(i - j) >= indexDifference, and
// abs(nums[i] - nums[j]) >= valueDifference
// Return an integer array answer, where answer = [i, j] if there are two such indices, and answer = [-1, -1] otherwise.
// If there are multiple choices for the two indices, return any of them.
//
// Note: i and j may be equal.
//
// Example 1:
//
// Input: nums = [5,1,4,1], indexDifference = 2, valueDifference = 4
// Output: [0,3]
// Explanation: In this example, i = 0 and j = 3 can be selected.
// abs(0 - 3) >= 2 and abs(nums[0] - nums[3]) >= 4.
// Hence, a valid answer is [0,3].
// [3,0] is also a valid answer.
//
// Example 2:
//
// Input: nums = [2,1], indexDifference = 0, valueDifference = 0
// Output: [0,0]
// Explanation: In this example, i = 0 and j = 0 can be selected.
// abs(0 - 0) >= 0 and abs(nums[0] - nums[0]) >= 0.
// Hence, a valid answer is [0,0].
// Other valid answers are [0,1], [1,0], and [1,1].
//
// Example 3:
//
// Input: nums = [1,2,3], indexDifference = 2, valueDifference = 4
// Output: [-1,-1]
// Explanation: In this example, it can be shown that it is impossible to find two indices that satisfy both conditions.
// Hence, [-1,-1] is returned.
//
// Constraints:
//
// 1 <= n == nums.length <= 10^5
// 0 <= nums[i] <= 10^9
// 0 <= indexDifference <= 10^5
// 0 <= valueDifference <= 10^9
//

struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut lo_val_idx = 0;
        let mut hi_val_idx = 0;
        let n = nums.len();
        if index_difference as usize >= n {
            return vec![-1, -1];
        }
        for i in index_difference as usize..n {
            let v = nums[i];
            let j = i - index_difference as usize;
            let (mut lo_val, mut hi_val) = (nums[lo_val_idx], nums[hi_val_idx]);

            if nums[j] < lo_val {
                lo_val_idx = j;
                lo_val = nums[j];
            }
            if nums[j] > hi_val {
                hi_val_idx = j;
                hi_val = nums[j];
            }
            if v - lo_val >= value_difference {
                return vec![lo_val_idx as i32, i as i32];
            }
            if hi_val - v >= value_difference {
                return vec![hi_val_idx as i32, i as i32];
            }
        }
        vec![-1, -1]
    }
}

#[test]
fn test() {
    let nums = vec![5, 1, 4, 1];
    let index_difference = 2;
    let value_difference = 4;
    let ans = vec![0, 3];
    assert_eq!(Solution::find_indices(nums, index_difference, value_difference), ans);

    let nums = vec![2, 1];
    let index_difference = 0;
    let value_difference = 0;
    let ans = vec![0, 0];
    assert_eq!(Solution::find_indices(nums, index_difference, value_difference), ans);

    let nums = vec![1, 2, 3];
    let index_difference = 2;
    let value_difference = 4;
    let ans = vec![-1, -1];
    assert_eq!(Solution::find_indices(nums, index_difference, value_difference), ans);
}
