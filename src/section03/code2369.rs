#![allow(dead_code)]

/*

// 2369. Check if There is a Valid Partition For The Array
// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
// https://leetcode.cn/problems/check-if-there-is-a-valid-partition-for-the-array/
//
// Medium
//
// You are given a 0-indexed integer array nums. You have to partition the array into one or more contiguous subarrays.

We call a partition of the array valid if each of the obtained subarrays satisfies one of the following conditions:

    The subarray consists of exactly 2 equal elements. For example, the subarray [2,2] is good.
    The subarray consists of exactly 3 equal elements. For example, the subarray [4,4,4] is good.
    The subarray consists of exactly 3 consecutive increasing elements, that is, the difference between adjacent elements is 1. For example, the subarray [3,4,5] is good, but the subarray [1,3,5] is not.

Return true if the array has at least one valid partition. Otherwise, return false.

Example 1:

Input: nums = [4,4,4,5,6]
Output: true
Explanation: The array can be partitioned into the subarrays [4,4] and [4,5,6].
This partition is valid, so we return true.

Example 2:

Input: nums = [1,1,1,2]
Output: false
Explanation: There is no valid partition for this array.

Constraints:

    2 <= nums.length <= 10^5
    1 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let eq_pair = nums[0] == nums[1];
        if nums.len() == 2 {
            return eq_pair;
        }
        let mut dp = vec![false; nums.len() + 1];
        dp[0] = true;
        dp[1] = false;
        dp[2] = eq_pair;
        for (v, i) in nums.windows(3).zip(3..) {
            if let &[a, b, c] = v {
                dp[i] = dp[i - 2] && b == c || dp[i - 3] && (a == b && b == c || b - a == 1 && c - b == 1);
                if !dp[i - 3..i].iter().any(|&x| x) {
                    break;
                }
            }
        }
        *dp.last().unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 4, 4, 5, 6], true),
        (vec![1, 1, 1, 2], false),
        (vec![1, 1, 1, 2, 2, 2], true),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::valid_partition(nums), expected);
    }
}
