#![allow(dead_code)]

/*

// 2009. Minimum Number of Operations to Make Array Continuous
// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/
// https://leetcode.cn/problems/minimum-number-of-operations-to-make-array-continuous/
//
// Hard
//
// You are given an integer array nums. In one operation, you can replace any element in nums with any integer.

nums is considered continuous if both of the following conditions are fulfilled:

    All elements in nums are unique.
    The difference between the maximum element and the minimum element in nums equals nums.length - 1.

For example, nums = [4, 2, 5, 3] is continuous, but nums = [1, 2, 3, 5, 6] is not continuous.

Return the minimum number of operations to make nums continuous.

Example 1:

Input: nums = [4,2,5,3]
Output: 0
Explanation: nums is already continuous.

Example 2:

Input: nums = [1,2,3,5,6]
Output: 1
Explanation: One possible solution is to change the last element to 4.
The resulting array is [1,2,3,5,4], which is continuous.

Example 3:

Input: nums = [1,10,100,1000]
Output: 3
Explanation: One possible solution is to:
- Change the second element to 2.
- Change the third element to 3.
- Change the fourth element to 4.
The resulting array is [1,2,3,4], which is continuous.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        let mut ans = n as i32;
        nums.sort_unstable();
        nums.dedup();
        let m = nums.len();
        let mut j = 0;
        for i in 0..m {
            while j < m && nums[j] < nums[i] + n as i32 {
                j += 1;
            }
            ans = ans.min(n as i32 - j as i32 + i as i32);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 2, 5, 3], 0),
        (vec![1, 2, 3, 5, 6], 1),
        (vec![1, 10, 100, 1000], 3),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::min_operations(nums), expected);
    }
}
