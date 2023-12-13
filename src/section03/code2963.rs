#![allow(dead_code)]

// 2963. Count the Number of Good Partitions
// https://leetcode.com/problems/count-the-number-of-good-partitions/
// https://leetcode.cn/problems/count-the-number-of-good-partitions/
//
// Hard
//
// You are given a 0-indexed array nums consisting of positive integers.
//
// A partition of an array into one or more contiguous subarrays is called good
// if no two subarrays contain the same number.
//
// Return the total number of good partitions of nums.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
// Output: 8
// Explanation: The 8 possible good partitions are: ([1], [2], [3], [4]), ([1], [2], [3,4]),
// ([1], [2,3], [4]), ([1], [2,3,4]), ([1,2], [3], [4]), ([1,2], [3,4]), ([1,2,3], [4]), and ([1,2,3,4]).
//
// Example 2:
//
// Input: nums = [1,1,1,1]
// Output: 1
// Explanation: The only possible good partition is: ([1,1,1,1]).
//
// Example 3:
//
// Input: nums = [1,2,1,3]
// Output: 2
// Explanation: The 2 possible good partitions are: ([1,2,1], [3]) and ([1,2,1,3]).
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        let mut res: i64 = 1;
        let mut last: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mod_num: i64 = 1e9 as i64 + 7;

        for (i, val) in nums.iter().enumerate() {
            last.insert(*val, i as i32);
        }

        let mut j = 0;
        for (i, val) in nums.iter().enumerate() {
            if i as i32 > j {
                res = (res * 2) % mod_num;
            }
            j = j.max(last[val]);
        }

        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 3, 4]), 8);
    assert_eq!(Solution::number_of_good_partitions(vec![1, 1, 1, 1]), 1);
    assert_eq!(Solution::number_of_good_partitions(vec![1, 2, 1, 3]), 2);
}
