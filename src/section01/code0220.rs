#![allow(dead_code)]

// 220. Contain Duplicate III
// https://leetcode.com/problems/contains-duplicate-iii/
// https://leetcode.cn/problems/contains-duplicate-iii/
//
// You are given an integer array nums and two integers indexDiff and valueDiff.
//
// Find a pair of indices (i, j) such that:
//
// - i != j,
// - abs(i - j) <= indexDiff.
// - abs(nums[i] - nums[j]) <= valueDiff, and
//
// Return true if such pair exists or false otherwise.
//
// Example 1:
//
// Input: nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
// Output: true
// Explanation: We can choose (i, j) = (0, 3).
// We satisfy the three conditions:
// i != j --> 0 != 3
// abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
// abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
//
// Example 2:
//
// Input: nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
// Output: false
// Explanation: After trying all the possible pairs (i, j), we cannot satisfy the three conditions, so we return false.
//
// Constraints:
//
// - 2 <= nums.length <= 105
// - -109 <= nums[i] <= 109
// - 1 <= indexDiff <= nums.length
// - 0 <= valueDiff <= 109
//

struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        if value_diff < 0 {
            return false;
        }
        let index_diff = index_diff as usize;
        let value_diff = value_diff as i64;
        let mut bts = std::collections::BTreeSet::<i64>::new();
        for i in 0..nums.len() {
            if i > index_diff {
                bts.remove(&(nums[i - 1 - index_diff] as i64));
            }
            if bts
                .range(nums[i] as i64 - value_diff..=nums[i] as i64 + value_diff)
                .next()
                .is_some()
            {
                return true;
            }
            bts.insert(nums[i] as i64);
        }
        false
    }
}

#[test]
fn test_contains_nearby_almost_duplicate() {
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0), true);
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3), false);
}
