#![allow(dead_code)]

// 3684. Maximize Sum of At Most K Distinct Elements
// https://leetcode.com/problems/maximize-sum-of-at-most-k-distinct-elements/
// https://leetcode.cn/problems/maximize-sum-of-at-most-k-distinct-elements/
//
// Easy
//
// You are given a positive integer array nums and an integer k.
//
// Choose at most k elements from nums so that their sum is maximized. However, the chosen numbers must be distinct.
//
// Return an array containing the chosen numbers in strictly descending order.
//
// Example 1:
//
// Input: nums = [84,93,100,77,90], k = 3
//
// Output: [100,93,90]
//
// Explanation:
//
// The maximum sum is 283, which is attained by choosing 93, 100 and 90. We rearrange them in strictly descending order as [100, 93, 90].
//
// Example 2:
//
// Input: nums = [84,93,100,77,93], k = 3
//
// Output: [100,93,84]
//
// Explanation:
//
// The maximum sum is 277, which is attained by choosing 84, 93 and 100. We rearrange them in strictly descending
// order as [100, 93, 84]. We cannot choose 93, 100 and 93 because the chosen numbers must be distinct.
//
// Example 3:
//
// Input: nums = [1,1,1,2,2,2], k = 6
//
// Output: [2,1]
//
// Explanation:
//
// The maximum sum is 3, which is attained by choosing 1 and 2. We rearrange them in strictly descending order as [2, 1].
//
// Constraints:
//
// 1 <= nums.length <= 100
// 1 <= nums[i] <= 10^9
// 1 <= k <= nums.length
//

struct Solution;

impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BTreeSet;
        nums.into_iter()
            .collect::<BTreeSet<i32>>()
            .into_iter()
            .rev()
            .take(k as usize)
            .collect::<Vec<i32>>()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![84, 93, 100, 77, 90], 3, vec![100, 93, 90]),
        (vec![84, 93, 100, 77, 93], 3, vec![100, 93, 84]),
        (vec![1, 1, 1, 2, 2, 2], 6, vec![2, 1]),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::max_k_distinct(nums, k), expected);
    }
}
