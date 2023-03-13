#![allow(dead_code)]

// 2488. Count Number of Nice Subarrays
// https://leetcode.com/problems/count-number-of-nice-subarrays/
// https://leetcode.cn/problems/count-number-of-nice-subarrays/
//
// You are given an array nums of size n consisting of distinct integers from 1 to n and a positive integer k.
//
// Return the number of non-empty subarrays in nums that have a median equal to k.
//
// Note:
//
// - The median of an array is the middle element after sorting the array in ascending order.
//   If the array is of even length, the median is the left middle element.
// - For example, the median of [2,3,1,4] is 2, and the median of [8,4,3,5,1] is 4.
// - A subarray is a contiguous part of an array.
//
//Example 1:
//
// Input: nums = [3,2,1,4,5], k = 4
// Output: 3
// Explanation: The subarrays that have a median equal to 4 are: [4], [4,5] and [1,4,5].
//
// Example 2:
//
// Input: nums = [2,3,1], k = 3
// Output: 1
// Explanation: [3] is the only subarray that has a median equal to 3.
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 10^5
// - 1 <= nums[i], k <= n
// - The integers in nums are distinct.
//

struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut i = 0;
        while nums[i] != k {
            i += 1;
        }

        let (mut left, mut right) = (HashMap::<i32, i32>::new(), HashMap::<i32, i32>::new());

        let mut cnt = 0;
        *right.entry(cnt).or_default() += 1;
        for &item in nums.iter().skip(i + 1) {
            cnt += if item > k { 1 } else { -1 };
            *right.entry(cnt).or_default() += 1;
        }

        cnt = 0;
        *left.entry(cnt).or_default() += 1;
        for j in (0..i).rev() {
            cnt += if nums[j] > k { 1 } else { -1 };
            *left.entry(cnt).or_default() += 1;
        }

        let mut ret = 0;
        for (key, val) in left {
            if let Some(cnt) = right.get(&(-key)) {
                ret += val * cnt;
            }
            if let Some(cnt) = right.get(&(1 - key)) {
                ret += val * cnt;
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
    assert_eq!(Solution::number_of_subarrays(vec![2, 3, 1], 3), 1);
}
