#![allow(dead_code)]

// 330. Patching Array
// https://leetcode.com/problems/patching-array/
//
// Given a sorted integer array nums and an integer n, add/patch elements to the array such that any number
// in the range [1, n] inclusive can be formed by the sum of some elements in the array.
//
// Return the minimum number of patches required.
//
// Example 1:
//
// Input: nums = [1,3], n = 6
// Output: 1
// Explanation:
// Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
// Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
// Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
// So we only need 1 patch.
//
// Example 2:
//
// Input: nums = [1,5,10], n = 20
// Output: 2
// Explanation: The two patches can be [2, 4].
//
// Example 3:
//
// Input: nums = [1,2,2], n = 5
// Output: 0
//
// Constraints:
//
// - 1 <= nums.length <= 1000
// - 1 <= nums[i] <= 104
// - nums is sorted in ascending order.
// - 1 <= n <= 231 - 1
//

struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut index = 0;
        let mut count = 0;
        let mut value_need = 1;
        while value_need <= n as i64 {
            if index < nums.len() && value_need >= nums[index] {
                value_need += nums[index];
                index += 1;
            } else {
                value_need += value_need;
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test_min_patches() {
    assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
    assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}
