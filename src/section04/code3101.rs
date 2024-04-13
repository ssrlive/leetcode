#![allow(dead_code)]

// 3101. Count Alternating Subarrays
// https://leetcode.com/problems/count-alternating-subarrays/
// https://leetcode.cn/problems/count-alternating-subarrays/
//
// Medium
//
// You are given a binary array nums.
//
// We call a subarray alternating if no two adjacent elements in the subarray have the same value.
//
// Return the number of alternating subarrays in nums.
//
// Example 1:
//
// Input: nums = [0,1,1,1]
//
// Output: 5
//
// Explanation:
//
// The following subarrays are alternating: [0], [1], [1], [1], and [0,1].
//
// Example 2:
//
// Input: nums = [1,0,1,0]
//
// Output: 10
//
// Explanation:
//
// Every subarray of the array is alternating. There are 10 possible subarrays that we can choose.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     nums[i] is either 0 or 1.
//

struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let (mut res, mut pos_l, mut prev_v) = (0, 0, -1);
        for (pos_r, &num) in nums.iter().enumerate() {
            if num == prev_v {
                pos_l = pos_r;
            }
            prev_v = num;
            res += (pos_r - pos_l + 1) as i64;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 1, 1];
    let res = 5;
    assert_eq!(Solution::count_alternating_subarrays(nums), res);

    let nums = vec![1, 0, 1, 0];
    let res = 10;
    assert_eq!(Solution::count_alternating_subarrays(nums), res);
}
