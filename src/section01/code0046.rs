#![allow(dead_code)]

// 46. Permutations
// https://leetcode.com/problems/permutations/
// https://leetcode.cn/problems/permutations/
//
// Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// Example 2:
//
// Input: nums = [0,1]
// Output: [[0,1],[1,0]]
//
// Example 3:
//
// Input: nums = [1]
// Output: [[1]]
//
// Constraints:
//
// - 1 <= nums.length <= 6
// - -10 <= nums[i] <= 10
// - All the integers of nums are unique.
//

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![nums.clone()];
        let size = nums.len();
        for i in 0..size - 1 {
            for idx in 0..result.len() {
                for j in i + 1..size {
                    let mut tmp = result[idx].clone();
                    tmp.swap(i, j);
                    result.push(tmp);
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        ),
        (vec![0, 1], vec![vec![0, 1], vec![1, 0]]),
        (vec![1], vec![vec![1]]),
    ];
    for (nums, expected) in cases {
        let mut result = Solution::permute(nums);
        result.sort();
        assert_eq!(result, expected);
    }
}
