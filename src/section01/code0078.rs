#![allow(dead_code)]

// 78. Subsets
// https://leetcode.com/problems/subsets/
// https://leetcode.cn/problems/subsets/
//
// Medium
//
// Given an integer array nums of unique elements, return all possible subsets (the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in any order.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//
// Example 2:
//
// Input: nums = [0]
// Output: [[],[0]]
//
// Constraints:
//
// - 1 <= nums.length <= 10
// - -10 <= nums[i] <= 10
// - All the numbers of nums are unique.
//

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        if nums.is_empty() {
            return vec![vec![]];
        }
        let last = nums.pop().unwrap();
        let remain_subsets = Self::subsets(nums);
        let mut res = remain_subsets.clone();
        for mut v in remain_subsets {
            v.push(last);
            res.push(v);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 2, 3],
            vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]],
        ),
        (vec![0], vec![vec![], vec![0]]),
    ];
    for (nums, expect) in cases {
        assert_eq!(Solution::subsets(nums), expect);
    }
}
