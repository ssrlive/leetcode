#![allow(dead_code)]

// 219. Contains Duplicate II
// https://leetcode.com/problems/contains-duplicate-ii/
// https://leetcode.cn/problems/contains-duplicate-ii/
//
// Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array
// such that nums[i] == nums[j] and abs(i - j) <= k.
//
// Example 1:
// Input: nums = [1,2,3,1], k = 3
// Output: true
//
// Example 2:
// Input: nums = [1,0,1,1], k = 1
// Output: true
//
// Example 3:
// Input: nums = [1,2,3,1,2,3], k = 2
// Output: false
//
// Constraints:
// 1 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
// 0 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                if i - j <= k as usize {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}

#[test]
fn test_contains_nearby_duplicate() {
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true);
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
    assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
}
