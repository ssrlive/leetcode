#![allow(dead_code)]

// 3487. Maximum Unique Subarray Sum After Deletion
// https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/
// https://leetcode.cn/problems/maximum-unique-subarray-sum-after-deletion/
//
// Easy
//
// You are given an integer array nums.
//
// You are allowed to delete any number of elements from nums without making it empty. After performing the deletions, select a subarray of nums such that:
//
//     All elements in the subarray are unique.
//     The sum of the elements in the subarray is maximized.
//
// Return the maximum sum of such a subarray.
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5]
//
// Output: 15
//
// Explanation:
//
// Select the entire array without deleting any element to obtain the maximum sum.
//
// Example 2:
//
// Input: nums = [1,1,0,1,1]
//
// Output: 1
//
// Explanation:
//
// Delete the element nums[0] == 1, nums[1] == 1, nums[2] == 0, and nums[3] == 1. Select the entire array [1] to obtain the maximum sum.
//
// Example 3:
//
// Input: nums = [1,2,-1,-2,1,0,-1]
//
// Output: 3
//
// Explanation:
//
// Delete the elements nums[2] == -1 and nums[3] == -2, and select the subarray [2, 1] from [1, 2, 1, 0, -1] to obtain the maximum sum.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     -100 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut mx = i32::MIN;
        let mut s = std::collections::HashSet::new();
        for &nums_i in &nums {
            mx = mx.max(nums_i);
            if nums_i <= 0 || s.contains(&nums_i) {
                continue;
            }
            sum += nums_i;
            s.insert(nums_i);
        }
        if mx <= 0 { mx } else { sum }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let res = 15;
    assert_eq!(Solution::max_sum(nums), res);

    let nums = vec![1, 1, 0, 1, 1];
    let res = 1;
    assert_eq!(Solution::max_sum(nums), res);

    let nums = vec![1, 2, -1, -2, 1, 0, -1];
    let res = 3;
    assert_eq!(Solution::max_sum(nums), res);
}
