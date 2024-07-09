#![allow(dead_code)]

// 3190. Find Minimum Operations to Make All Elements Divisible by Three
// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/
// https://leetcode.cn/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/
//
// Easy
//
// You are given an integer array nums. In one operation, you can add or subtract 1 from any element of nums.
//
// Return the minimum number of operations to make all elements of nums divisible by 3.
//
// Example 1:
//
// Input: nums = [1,2,3,4]
//
// Output: 3
//
// Explanation:
//
// All array elements can be made divisible by 3 using 3 operations:
//
//     Subtract 1 from 1.
//     Add 1 to 2.
//     Subtract 1 from 4.
//
// Example 2:
//
// Input: nums = [3,6,9]
//
// Output: 0
//
// Constraints:
//
//     1 <= nums.length <= 50
//     1 <= nums[i] <= 50
//

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|x| x % 3 != 0).count() as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
    assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
    assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 6);
    assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 7);
}
