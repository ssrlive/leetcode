#![allow(dead_code)]

// 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
// https://leetcode.cn/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
//
// Medium
//
// Given an array of integers nums and an integer limit, return the size of the longest non-empty subarray such
// that the absolute difference between any two elements of this subarray is less than or equal to limit.
//
// Example 1:
//
// Input: nums = [8,2,4,7], limit = 4
// Output: 2
// Explanation: All subarrays are:
// [8] with maximum absolute diff |8-8| = 0 <= 4.
// [8,2] with maximum absolute diff |8-2| = 6 > 4.
// [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
// [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
// [2] with maximum absolute diff |2-2| = 0 <= 4.
// [2,4] with maximum absolute diff |2-4| = 2 <= 4.
// [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
// [4] with maximum absolute diff |4-4| = 0 <= 4.
// [4,7] with maximum absolute diff |4-7| = 3 <= 4.
// [7] with maximum absolute diff |7-7| = 0 <= 4.
// Therefore, the size of the longest subarray is 2.
//
// Example 2:
//
// Input: nums = [10,1,2,4,7,2], limit = 5
// Output: 4
// Explanation: The subarray [2,4,7,2] is the longest since the maximum absolute diff is |2-7| = 5 <= 5.
//
// Example 3:
//
// Input: nums = [4,2,2,2,4,4,2,2], limit = 0
// Output: 3
//
// Constraints:
//
// -    1 <= nums.length <= 10^5
// -    1 <= nums[i] <= 10^9
// -    0 <= limit <= 10^9
//

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut i = 0;
        let mut minq: Vec<i32> = vec![];
        let mut maxq: Vec<i32> = vec![];
        for num in nums.iter() {
            while !minq.is_empty() && num < minq.last().unwrap() {
                minq.pop();
            }
            while !maxq.is_empty() && num > maxq.last().unwrap() {
                maxq.pop();
            }
            minq.push(*num);
            maxq.push(*num);
            if maxq[0] - minq[0] > limit {
                if minq[0] == nums[i] {
                    minq.remove(0);
                }
                if maxq[0] == nums[i] {
                    maxq.remove(0);
                }
                i += 1;
            }
        }
        (nums.len() as i32) - (i as i32)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![8, 2, 4, 7], 4, 2),
        (vec![10, 1, 2, 4, 7, 2], 5, 4),
        (vec![4, 2, 2, 2, 4, 4, 2, 2], 0, 3),
    ];
    for (nums, limit, expected) in cases {
        assert_eq!(Solution::longest_subarray(nums, limit), expected);
    }
}
