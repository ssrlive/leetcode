#![allow(dead_code)]

// 3349. Adjacent Increasing Subarrays Detection I
// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i/
// https://leetcode.cn/problems/adjacent-increasing-subarrays-detection-i/
//
// Easy
//
// Given an array nums of n integers and an integer k, determine whether there exist two adjacent subarrays
// of length k such that both subarrays are strictly increasing. Specifically,
// check if there are two subarrays starting at indices a and b (a < b), where:
//
//     Both subarrays nums[a..a + k - 1] and nums[b..b + k - 1] are strictly increasing.
//     The subarrays must be adjacent, meaning b = a + k.
//
// Return true if it is possible to find two such subarrays, and false otherwise.
//
// Example 1:
//
// Input: nums = [2,5,7,8,9,2,3,4,3,1], k = 3
//
// Output: true
//
// Explanation:
//
//     The subarray starting at index 2 is [7, 8, 9], which is strictly increasing.
//     The subarray starting at index 5 is [2, 3, 4], which is also strictly increasing.
//     These two subarrays are adjacent, so the result is true.
//
// Example 2:
//
// Input: nums = [1,2,3,4,4,4,4,5,6,7], k = 5
//
// Output: false
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 < 2 * k <= nums.length
//     -1000 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let k = k as usize;
        let n = nums.len();
        for i in 1..n - 2 * k + 2 {
            let mut valid = true;
            for j in 0..k - 1 {
                if nums[i + j] <= nums[i + j - 1] || nums[i + j + k] <= nums[i + j + k - 1] {
                    valid = false;
                    break;
                }
            }
            if valid {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
    let k = 3;
    let res = true;
    assert_eq!(Solution::has_increasing_subarrays(nums, k), res);

    let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
    let k = 5;
    let res = false;
    assert_eq!(Solution::has_increasing_subarrays(nums, k), res);
}
