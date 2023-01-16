#![allow(dead_code)]

// 775. Global and Local Inversions
// https://leetcode.com/problems/global-and-local-inversions/
// https://leetcode.cn/problems/global-and-local-inversions/
//
// You are given an integer array nums of length n which represents a permutation of all the integers in the range [0, n - 1].
//
// The number of global inversions is the number of the different pairs (i, j) where:
//
// - 0 <= i < j < n
// - nums[i] > nums[j]
//
// The number of local inversions is the number of indices i where:
//
// - 0 <= i < n - 1
// - nums[i] > nums[i + 1]
//
// Return true if the number of global inversions is equal to the number of local inversions.
//
// Example 1:
//
// Input: nums = [1,0,2]
// Output: true
// Explanation: There is 1 global inversion and 1 local inversion.
//
// Example 2:
//
// Input: nums = [1,2,0]
// Output: false
// Explanation: There are 2 global inversions and 1 local inversion.
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 10^5
// - 0 <= nums[i] < n
// - All the integers of nums are unique.
// - nums is a permutation of all the numbers in the range [0, n - 1].
//

struct Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        fn _is_ideal_permutation(nums: Vec<i32>) -> Option<bool> {
            let mut max = 0;
            for i in 0..nums.len().checked_sub(2)? {
                max = std::cmp::max(max, *nums.get(i)?);
                if max > *nums.get(i + 2)? {
                    return Some(false);
                }
            }
            Some(true)
        }
        _is_ideal_permutation(nums).unwrap_or(true)
    }

    pub fn is_ideal_permutation2(nums: Vec<i32>) -> bool {
        nums.iter().enumerate().all(|(i, &x)| (x - i as i32).abs() <= 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
    assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
    assert_eq!(Solution::is_ideal_permutation(vec![0]), true);
}
