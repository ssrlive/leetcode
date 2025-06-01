#![allow(dead_code)]

// 3566. Partition Array into Two Equal Product Subsets
// https://leetcode.com/problems/partition-array-into-two-equal-product-subsets/
// https://leetcode.cn/problems/partition-array-into-two-equal-product-subsets/
//
// Medium
//
// You are given an integer array nums containing distinct positive integers and an integer target.
//
// Determine if you can partition nums into two non-empty disjoint subsets, with each element belonging
// to exactly one subset, such that the product of the elements in each subset is equal to target.
//
// Return true if such a partition exists and false otherwise.
// A subset of an array is a selection of elements of the array.
//
// Example 1:
//
// Input: nums = [3,1,6,8,4], target = 24
//
// Output: true
//
// Explanation: The subsets [3, 8] and [1, 6, 4] each have a product of 24. Hence, the output is true.
//
// Example 2:
//
// Input: nums = [2,5,3,7], target = 15
//
// Output: false
//
// Explanation: There is no way to partition nums into two non-empty disjoint subsets such that both subsets
// have a product of 15. Hence, the output is false.
//
// Constraints:
//
//     3 <= nums.length <= 12
//     1 <= target <= 10^15
//     1 <= nums[i] <= 100
//     All elements of nums are distinct.
//

struct Solution;

impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        fn solve(i: usize, prod: i128, target: i128, nums: &[i32]) -> bool {
            if prod > target {
                return false;
            }
            if prod == target {
                return true;
            }
            if i == nums.len() {
                return false;
            }

            if solve(i + 1, prod * nums[i] as i128, target, nums) {
                return true;
            }

            if solve(i + 1, prod, target, nums) {
                return true;
            }

            false
        }

        let target = target as i128;
        let mut total: i128 = 1;
        for &x in &nums {
            total *= x as i128;
            if total > target * target {
                return false;
            }
        }
        if total != target * target {
            return false;
        }
        solve(0, 1, target, &nums)
    }
}

#[test]
fn test() {
    let nums = vec![3, 1, 6, 8, 4];
    let target = 24;
    assert!(Solution::check_equal_partitions(nums, target));

    let nums = vec![2, 5, 3, 7];
    let target = 15;
    assert!(!Solution::check_equal_partitions(nums, target));

    let nums = vec![1, 2, 8];
    let target = 4;
    assert!(!Solution::check_equal_partitions(nums, target));
}
