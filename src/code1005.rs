#![allow(dead_code)]

// 1005. Maximize Sum Of Array After K Negations
// https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/
// https://leetcode.cn/problems/maximize-sum-of-array-after-k-negations/
//
// Given an integer array nums and an integer k, modify the array in the following way:
//
// choose an index i and replace nums[i] with -nums[i].
// You should apply this process exactly k times. You may choose the same index i multiple times.
//
// Return the largest possible sum of the array after modifying it in this way.
//
// Example 1:
//
// Input: nums = [4,2,3], k = 1
// Output: 5
// Explanation: Choose index 1 and nums becomes [4,-2,3].
//
// Example 2:
//
// Input: nums = [3,-1,0,2], k = 3
// Output: 6
// Explanation: Choose indices (1, 2, 2) and nums becomes [3,1,0,2].
//
// Example 3:
//
// Input: nums = [2,-3,-1,5,-4], k = 2
// Output: 13
// Explanation: Choose indices (1, 4) and nums becomes [2,3,-1,5,4].
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - -100 <= nums[i] <= 100
// - 1 <= k <= 10^4
//

struct Solution;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut k = k;
        let nums_rev = nums.iter().cloned().map(Reverse).collect::<Vec<_>>();
        let mut min_heap = BinaryHeap::from(nums_rev);

        while k > 0 {
            if let Some(Reverse(val)) = min_heap.pop() {
                min_heap.push(Reverse(-val));
            }
            k -= 1;
        }

        min_heap.into_iter().fold(0, |acc, v| {
            let Reverse(val) = v;
            acc + val
        })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    assert_eq!(Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    assert_eq!(Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
}
