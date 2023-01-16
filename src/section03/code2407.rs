#![allow(dead_code)]

// 2407. Longest Increasing Subsequence II
// https://leetcode.com/problems/longest-increasing-subsequence-ii/description/
// https://leetcode.cn/problems/longest-increasing-subsequence-ii/description/
//
// You are given an integer array nums and an integer k.
//
// Find the longest subsequence of nums that meets the following requirements:
//
// The subsequence is strictly increasing and
// The difference between adjacent elements in the subsequence is at most k.
// Return the length of the longest subsequence that meets the requirements.
//
// A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
//
// Example 1:
//
// Input: nums = [4,2,1,4,3,4,5,8,15], k = 3
// Output: 5
// Explanation:
// The longest subsequence that meets the requirements is [1,3,4,5,8].
// The subsequence has a length of 5, so we return 5.
// Note that the subsequence [1,3,4,5,8,15] does not meet the requirements because 15 - 8 = 7 is larger than 3.
//
// Example 2:
//
// Input: nums = [7,4,5,1,8,12,4,7], k = 5
// Output: 4
// Explanation:
// The longest subsequence that meets the requirements is [4,5,8,12].
// The subsequence has a length of 4, so we return 4.
//
// Example 3:
//
// Input: nums = [1,5], k = 1
// Output: 1
// Explanation:
// The longest subsequence that meets the requirements is [1].
// The subsequence has a length of 1, so we return 1.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i], k <= 10^5
//

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        fn update(tree: &mut [i32], idx: i32, val: i32) {
            let mut idx = idx as usize + 100_001;
            tree[idx] = val;

            while idx > 1 {
                tree[idx >> 1] = tree[idx].max(tree[idx ^ 1]);
                idx >>= 1;
            }
        }
        fn query(tree: &[i32], left: i32, right: i32) -> i32 {
            let mut left = left as usize + 100_001;
            let mut right = right as usize + 100_001;

            let mut res = 0;
            while left < right {
                if left & 1 == 1 {
                    res = res.max(tree[left]);
                    left += 1;
                }
                if right & 1 == 1 {
                    right -= 1;
                    res = res.max(tree[right]);
                }
                left /= 2;
                right /= 2;
            }
            res
        }

        let mut tree = [0; 2 * 100_000 + 2];

        let mut res = 0;
        for &val in &nums {
            let left = i32::max(0, val - k);

            let curr = query(&tree, left, val) + 1;
            res = res.max(curr);
            update(&mut tree, val, curr);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
    assert_eq!(Solution::length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
    assert_eq!(Solution::length_of_lis(vec![1, 5], 1), 1);
}
