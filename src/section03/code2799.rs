#![allow(dead_code)]

// 2799. Count Complete Subarrays in an Array
// https://leetcode.com/problems/count-complete-subarrays-in-an-array/
// https://leetcode.cn/problems/count-complete-subarrays-in-an-array/
//
// Medium
//
// You are given an array nums consisting of positive integers.
//
// We call a subarray of an array complete if the following condition is satisfied:
//
//     The number of distinct elements in the subarray is equal to the number of distinct elements in the whole array.
//
// Return the number of complete subarrays.
//
// A subarray is a contiguous non-empty part of an array.
//
//
//
// Example 1:
//
// Input: nums = [1,3,1,2,2]
// Output: 4
// Explanation: The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
//
// Example 2:
//
// Input: nums = [5,5,5,5]
// Output: 10
// Explanation: The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
//
// Constraints:
//
//     1 <= nums.length <= 1000
//     1 <= nums[i] <= 2000
//

struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let (_n, mut j, mut ret) = (nums.len(), 0, 0);
        let cnt = nums.clone().into_iter().collect::<HashSet<i32>>().len();
        let mut mp = HashMap::<i32, i32>::new();

        for i in 0..nums.len() {
            if i > 0 {
                if *mp.get(&nums[i - 1]).unwrap() == 1 {
                    mp.remove(&nums[i - 1]);
                } else {
                    *mp.entry(nums[i - 1]).or_insert(0) -= 1;
                }
            }
            while mp.len() < cnt && j < nums.len() {
                *mp.entry(nums[j]).or_insert(0) += 1;
                j += 1;
            }
            if mp.len() < cnt {
                break;
            }
            ret += (nums.len() - j + 1) as i32;
        }

        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
    assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
}
