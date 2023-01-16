#![allow(dead_code)]

// 2537. Count the Number of Good Subarrays
// https://leetcode.com/problems/count-the-number-of-good-subarrays/
// https://leetcode.cn/problems/count-the-number-of-good-subarrays/
//
// Given an integer array nums and an integer k, return the number of good subarrays of nums.
//
// A subarray arr is good if it there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,1,1,1,1], k = 10
// Output: 1
// Explanation: The only good subarray is the array nums itself.
//
// Example 2:
//
// Input: nums = [3,1,4,3,2,2,4], k = 2
// Output: 4
// Explanation: There are 4 different good subarrays:
// - [3,1,4,3,2,2] that has 2 pairs.
// - [3,1,4,3,2,2,4] that has 3 pairs.
// - [1,4,3,2,2,4] that has 2 pairs.
// - [4,3,2,2,4] that has 2 pairs.
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i], k <= 10^9
//

struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        let mut mp = HashMap::<i32, i32>::new();
        let (n, mut j) = (nums.len(), 0);
        let (mut ret, mut count, k) = (0, 0, k as i64);

        for i in 0..n {
            while j < n && count < k {
                if let Some(cnt) = mp.get(&nums[j]) {
                    count += *cnt as i64;
                }
                *mp.entry(nums[j]).or_insert(0) += 1;
                j += 1;
            }
            if count < k {
                break;
            }
            ret += (n - j + 1) as i64;
            *mp.entry(nums[i]).or_insert(0) -= 1;
            if let Some(cnt) = mp.get(&nums[i]) {
                count -= *cnt as i64;
            }
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
    assert_eq!(Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
}
