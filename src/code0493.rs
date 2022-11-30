#![allow(dead_code)]

// 493. Reverse Pairs
// https://leetcode.com/problems/reverse-pairs/
//
// Given an integer array nums, return the number of reverse pairs in the array.
//
// A reverse pair is a pair (i, j) where:
//
// - 0 <= i < j < nums.length and
// - nums[i] > 2 * nums[j].
//
// Example 1:
//
// Input: nums = [1,3,2,3,1]
// Output: 2
// Explanation: The reverse pairs are:
// (1, 4) --> nums[1] = 3, nums[4] = 1, 3 > 2 * 1
// (3, 4) --> nums[3] = 3, nums[4] = 1, 3 > 2 * 1
//
// Example 2:
//
// Input: nums = [2,4,3,5,1]
// Output: 3
// Explanation: The reverse pairs are:
// (1, 4) --> nums[1] = 4, nums[4] = 1, 4 > 2 * 1
// (2, 4) --> nums[2] = 3, nums[4] = 1, 3 > 2 * 1
// (3, 4) --> nums[3] = 5, nums[4] = 1, 5 > 2 * 1
//
// Constraints:
//
// - 1 <= nums.length <= 5 * 104
// - -231 <= nums[i] <= 231 - 1
//

struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        Self::_reverse_pairs(&nums) as i32
    }

    pub fn _reverse_pairs(nums: &[i64]) -> usize {
        let mut pairs = 0;
        let mut sort_nums = vec![nums[0]];
        for &item in nums.iter().skip(1) {
            pairs += sort_nums.len() - Self::binary_search_insert(&mut sort_nums, 2 * item, false);
            Self::binary_search_insert(&mut sort_nums, item, true);
        }
        pairs
    }

    fn binary_search_insert(nums: &mut Vec<i64>, val: i64, insert: bool) -> usize {
        let mut l = 0;
        let mut r = nums.len() as i64 - 1;
        while l <= r {
            let m = (l + r) >> 1;
            if nums[m as usize] > val {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        if insert {
            nums.insert((r + 1) as usize, val);
        }
        (r + 1) as usize
    }
}

#[test]
fn test_reverse_pairs() {
    let nums = vec![1, 3, 2, 3, 1];
    let result = Solution::reverse_pairs(nums);
    assert_eq!(result, 2);

    let nums = vec![2, 4, 3, 5, 1];
    let result = Solution::reverse_pairs(nums);
    assert_eq!(result, 3);
}
