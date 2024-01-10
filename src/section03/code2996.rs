#![allow(dead_code)]

// 2996. Smallest Missing Integer Greater Than Sequential Prefix Sum
// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum/
// https://leetcode.cn/problems/smallest-missing-integer-greater-than-sequential-prefix-sum/
//
// Easy
//
// You are given a 0-indexed array of integers nums.
//
// A prefix nums[0..i] is sequential if, for all 1 <= j <= i, nums[j] = nums[j - 1] + 1.
// In particular, the prefix consisting only of nums[0] is sequential.
//
// Return the smallest integer x missing from nums such that x is greater than or equal to the sum of the longest sequential prefix.
//
// Example 1:
//
// Input: nums = [1,2,3,2,5]
// Output: 6
// Explanation: The longest sequential prefix of nums is [1,2,3] with a sum of 6. 6 is not in the array,
// therefore 6 is the smallest missing integer greater than or equal to the sum of the longest sequential prefix.
//
// Example 2:
//
// Input: nums = [3,4,5,1,12,14,13]
// Output: 15
// Explanation: The longest sequential prefix of nums is [3,4,5] with a sum of 12. 12, 13,
// and 14 belong to the array while 15 does not.
// Therefore 15 is the smallest missing integer greater than or equal to the sum of the longest sequential prefix.
//
// Constraints:
//
//     1 <= nums.length <= 50
//     1 <= nums[i] <= 50
//

struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sequential_prefix = 0;
        for i in 1..nums.len() {
            if nums[i - 1] + 1 != nums[i] {
                sequential_prefix = i;
                break;
            }
        }
        let sum_sequential_prefix: i32 = if sequential_prefix == 0 {
            nums.iter().sum()
        } else {
            nums.get(..sequential_prefix).unwrap().iter().sum()
        };
        for i in sum_sequential_prefix..sum_sequential_prefix + nums.len() as i32 {
            if !nums.contains(&i) {
                return i;
            }
        }
        nums.iter().max().unwrap() + 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
    assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
}
