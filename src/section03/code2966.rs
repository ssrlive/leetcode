#![allow(dead_code)]

// 2966. Divide Array Into Arrays With Max Difference
// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/
// https://leetcode.cn/problems/divide-array-into-arrays-with-max-difference/
//
// Medium
//
// You are given an integer array nums of size n and a positive integer k.
//
// Divide the array into one or more arrays of size 3 satisfying the following conditions:
//
//     Each element of nums should be in exactly one array.
//     The difference between any two elements in one array is less than or equal to k.
//
// Return a 2D array containing all the arrays. If it is impossible to satisfy the conditions,
// return an empty array. And if there are multiple answers, return any of them.
//
// Example 1:
//
// Input: nums = [1,3,4,8,7,9,3,5,1], k = 2
// Output: [[1,1,3],[3,4,5],[7,8,9]]
// Explanation: We can divide the array into the following arrays: [1,1,3], [3,4,5] and [7,8,9].
// The difference between any two elements in each array is less than or equal to 2.
// Note that the order of elements is not important.
//
// Example 2:
//
// Input: nums = [1,3,3,2,7,3], k = 3
// Output: []
// Explanation: It is not possible to divide the array satisfying all the conditions.
//
// Constraints:
//
//     n == nums.length
//     1 <= n <= 10^5
//     n is a multiple of 3.
//     1 <= nums[i] <= 10^5
//     1 <= k <= 10^5
//

struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = Vec::new();
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(3) {
            let (v1, v2, v3) = (nums[i], nums[i + 1], nums[i + 2]);
            if v3 - v1 > k {
                return Vec::new();
            }
            res.push(vec![v1, v2, v3]);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2),
        vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]
    );
    assert_eq!(Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3), Vec::<Vec<i32>>::new());
}
