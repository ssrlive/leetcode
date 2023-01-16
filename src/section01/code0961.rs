#![allow(dead_code)]

// 961. N-Repeated Element in Size 2N Array
// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
// https://leetcode.cn/problems/n-repeated-element-in-size-2n-array/
//
// You are given an integer array nums with the following properties:
//
// nums.length == 2 * n.
// nums contains n + 1 unique elements.
// Exactly one element of nums is repeated n times.
//
// Return the element that is repeated n times.
//
// Example 1:
//
// Input: nums = [1,2,3,3]
// Output: 3
//
// Example 2:
//
// Input: nums = [2,1,2,5,3,2]
// Output: 2
//
// Example 3:
//
// Input: nums = [5,1,5,2,5,3,5,4]
// Output: 5
//
// Constraints:
//
// - 2 <= n <= 5000
// - nums.length == 2 * n
// - 0 <= nums[i] <= 10^4
// - nums contains n + 1 unique elements and one of them is repeated exactly n times.
//

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for n in &nums {
            if !set.insert(*n) {
                return *n;
            }
        }
        nums[0]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 3], 3),
        (vec![2, 1, 2, 5, 3, 2], 2),
        (vec![5, 1, 5, 2, 5, 3, 5, 4], 5),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::repeated_n_times(nums), expected);
    }
}
