#![allow(dead_code)]

// 915. Partition Array into Disjoint Intervals
// https://leetcode.com/problems/partition-array-into-disjoint-intervals/
// https://leetcode.cn/problems/partition-array-into-disjoint-intervals/
//
// Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:
//
// Every element in left is less than or equal to every element in right.
// left and right are non-empty.
// left has the smallest possible size.
// Return the length of left after such a partitioning.
//
// Test cases are generated such that partitioning exists.
//
// Example 1:
//
// Input: nums = [5,0,3,8,6]
// Output: 3
// Explanation: left = [5,0,3], right = [8,6]
//
// Example 2:
//
// Input: nums = [1,1,1,0,6,12]
// Output: 4
// Explanation: left = [1,1,1,0], right = [6,12]
//
// Constraints:
//
// - 2 <= nums.length <= 10^5
// - 0 <= nums[i] <= 10^6
// - There is at least one valid answer for the given input.
//

struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let max_l = {
            let mut res = vec![];
            let mut max = 0;
            for &num in nums.iter() {
                max = max.max(num);
                res.push(max);
            }
            res
        };
        let min_r = {
            let mut res = vec![];
            let mut min = std::i32::MAX;
            for &num in nums.iter().rev() {
                min = min.min(num);
                res.push(min);
            }
            res.reverse();
            res
        };
        for i in 0..(nums.len() - 1) {
            if max_l[i] <= min_r[i + 1] {
                return (i + 1) as i32;
            }
        }
        panic!()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
}
