#![allow(dead_code)]

// 3644. Maximum K to Sort a Permutation
// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/
// https://leetcode.cn/problems/maximum-k-to-sort-a-permutation/
//
// Medium
//
// You are given an integer array nums of length n, where nums is a permutation of the numbers in the range [0..n - 1].
//
// You may swap elements at indices i and j only if nums[i] AND nums[j] == k, where AND denotes the bitwise AND operation and k is a non-negative integer.
//
// Return the maximum value of k such that the array can be sorted in non-decreasing order using any number of such swaps. If nums is already sorted, return 0.
//
// Example 1:
//
// Input: nums = [0,3,2,1]
//
// Output: 1
//
// Explanation:
//
// Choose k = 1. Swapping nums[1] = 3 and nums[3] = 1 is allowed since nums[1] AND nums[3] == 1, resulting in a sorted permutation: [0, 1, 2, 3].
//
// Example 2:
//
// Input: nums = [0,1,3,2]
//
// Output: 2
//
// Explanation:
//
// Choose k = 2. Swapping nums[2] = 3 and nums[3] = 2 is allowed since nums[2] AND nums[3] == 2, resulting in a sorted permutation: [0, 1, 2, 3].
//
// Example 3:
//
// Input: nums = [3,2,1,0]
//
// Output: 0
//
// Explanation:
//
// Only k = 0 allows sorting since no greater k allows the required swaps where nums[i] AND nums[j] == k.
//
// Constraints:
//
// 1 <= n == nums.length <= 105
// 0 <= nums[i] <= n - 1
// nums is a permutation of integers from 0 to n - 1.
//

struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for (i, &num) in nums.iter().enumerate() {
            if i as i32 != num {
                ans &= num & i as i32;
            }
        }
        if ans == -1 { 0 } else { ans }
    }
}

#[test]
fn test() {
    let nums1 = vec![0, 3, 2, 1];
    assert_eq!(Solution::sort_permutation(nums1), 1);

    let nums2 = vec![0, 1, 3, 2];
    assert_eq!(Solution::sort_permutation(nums2), 2);

    let nums3 = vec![3, 2, 1, 0];
    assert_eq!(Solution::sort_permutation(nums3), 0);

    /*
        Input
    nums =
    [0,1,3,2,4,6,5]

    Use Testcase
    Expected
    0
         */
    let nums4 = vec![0, 1, 3, 2, 4, 6, 5];
    assert_eq!(Solution::sort_permutation(nums4), 0);
}
