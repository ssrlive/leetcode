#![allow(dead_code)]

// 1424. Diagonal Traverse II
// https://leetcode.com/problems/diagonal-traverse-ii/
// https://leetcode.cn/problems/diagonal-traverse-ii/
//
// Medium
//
// Given a 2D integer array nums, return all elements of nums in diagonal order as shown in the below images.
//
// Example 1:
//
// Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,4,2,7,5,3,8,6,9]
//
// Example 2:
//
// Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
// Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
//
// Constraints:
//
// -    1 <= nums.length <= 10^5
// -    1 <= nums[i].length <= 10^5
// -    1 <= sum(nums[i].length) <= 10^5
// -    1 <= nums[i][j] <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_key = 0;
        let mut map = std::collections::HashMap::new();
        let max_idx = nums.len() - 1;
        for (r, row) in nums.iter().rev().enumerate() {
            let idx = max_idx - r;
            for (c, &v) in row.iter().enumerate() {
                map.entry(idx + c).or_insert(vec![]).push(v);
                max_key = max_key.max(idx + c);
            }
        }
        let mut ans = vec![];
        for key in 0..=max_key {
            if let Some(vs) = map.get(&key) {
                ans.extend(vs);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![1, 4, 2, 7, 5, 3, 8, 6, 9]),
        (
            vec![vec![1, 2, 3, 4, 5], vec![6, 7], vec![8], vec![9, 10, 11], vec![12, 13, 14, 15, 16]],
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
        ),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::find_diagonal_order(nums), expected);
    }
}
