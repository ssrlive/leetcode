#![allow(dead_code)]

/*
// 2616. Minimize the Maximum Difference of Pairs
// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/
// https://leetcode.cn/problems/minimize-the-maximum-difference-of-pairs/
//
// Medium
//
// You are given a 0-indexed integer array nums and an integer p. Find p pairs of indices of nums such that the maximum difference amongst all the pairs is minimized. Also, ensure no index appears more than once amongst the p pairs.

Note that for a pair of elements at the index i and j, the difference of this pair is |nums[i] - nums[j]|, where |x| represents the absolute value of x.

Return the minimum maximum difference among all p pairs.

Example 1:

Input: nums = [10,1,2,7,1,3], p = 2
Output: 1
Explanation: The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5.
The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
Example 2:

Input: nums = [4,2,1,2], p = 1
Output: 0
Explanation: Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.

Constraints:

1 <= nums.length <= 10^5
0 <= nums[i] <= 10^9
0 <= p <= (nums.length)/2
*/

struct Solution;

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut left = 0;
        let mut right = nums[n - 1] - nums[0];
        while left < right {
            let mid = (left + right) / 2;
            let mut k = 0;
            let mut i = 1;
            while i < n && k < p {
                if nums[i] - nums[i - 1] <= mid {
                    k += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if k >= p {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![10, 1, 2, 7, 1, 3], 2, 1),
        (vec![4, 2, 1, 2], 1, 0),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, 1),
    ];
    for (nums, p, expect) in cases {
        assert_eq!(Solution::minimize_max(nums, p), expect);
    }
}
