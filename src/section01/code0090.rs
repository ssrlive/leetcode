#![allow(dead_code)]

// 90. Subsets II
// https://leetcode.com/problems/subsets-ii/
// https://leetcode.cn/problems/subsets-ii/
//
// Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in any order.
//
// Example 1:
//
// Input: nums = [1,2,2]
// Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
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
//

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        pub fn sub(opts: &Vec<i32>, i: usize, res: &mut HashSet<Vec<i32>>, nums: Vec<i32>) {
            if i == opts.len() {
                res.insert(nums);
                return;
            }

            let mut nums2 = nums.clone();
            nums2.push(opts[i]);

            let j = i + 1;
            sub(opts, j, res, nums);
            sub(opts, j, res, nums2);
        }

        let mut res = HashSet::new();

        let mut nums = nums;
        nums.sort_unstable();

        sub(&nums, 0, &mut res, vec![]);

        res.into_iter().collect::<Vec<Vec<i32>>>()
    }

    pub fn subsets_with_dup_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut res = HashSet::new();
        fn _subsets_with_dup_2(nums: Vec<i32>, res: &mut HashSet<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut nums = nums;
            if nums.is_empty() {
                res.insert(vec![]);
                return vec![vec![]];
            }
            let last = nums.pop().unwrap();
            let remain_subsets = _subsets_with_dup_2(nums, res);
            let mut mid = remain_subsets.clone();
            for mut v in remain_subsets {
                v.push(last);
                v.sort_unstable();
                res.insert(v.clone());
                mid.push(v);
            }
            mid
        }
        let _ = _subsets_with_dup_2(nums, &mut res);
        res.into_iter().collect::<Vec<Vec<i32>>>()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 2], vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]),
        (vec![0], vec![vec![], vec![0]]),
    ];

    for (nums, expect) in cases {
        let mut res = Solution::subsets_with_dup_2(nums);
        res.sort_unstable();
        assert_eq!(res, expect);
    }
}
