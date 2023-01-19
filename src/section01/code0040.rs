#![allow(dead_code)]

// 40. Combination Sum II
// https://leetcode.com/problems/combination-sum-ii/
// https://leetcode.cn/problems/combination-sum-ii/
//
// Given a collection of candidate numbers (candidates) and a target number (target),
// find all unique combinations in candidates where the candidate numbers sum to target.
//
// Each number in candidates may only be used once in the combination.
//
// Note: The solution set must not contain duplicate combinations.
//
// Example 1:
//
// Input: candidates = [10,1,2,7,6,1,5], target = 8
// Output:
// [
// [1,1,6],
// [1,2,5],
// [1,7],
// [2,6]
// ]
//
// Example 2:
//
// Input: candidates = [2,5,2,1,2], target = 5
// Output:
// [
// [1,2,2],
// [5]
// ]
//
// Constraints:
//
// - 1 <= candidates.length <= 100
// - 1 <= candidates[i] <= 50
// - 1 <= target <= 30
//

struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs_helper(candidates: &Vec<i32>, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, target: i32, idx: usize) {
            match target {
                0 => res.push(tmp.clone()),
                n if n < 0 => {}
                _ => {
                    for i in idx..candidates.len() {
                        let val = candidates[i];
                        if i > idx && val == candidates[i - 1] {
                            continue;
                        }
                        tmp.push(val);
                        dfs_helper(candidates, tmp, res, target - val, i + 1);
                        tmp.pop();
                    }
                }
            }
        }

        let mut res = vec![];
        let mut candidates = candidates;
        candidates.sort_unstable();
        dfs_helper(&candidates, &mut vec![], &mut res, target, 0);
        res
    }
}

#[test]
fn test() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let res = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
    assert_eq!(Solution::combination_sum2(candidates, target), res);

    let candidates = vec![2, 5, 2, 1, 2];
    let target = 5;
    let res = vec![vec![1, 2, 2], vec![5]];
    assert_eq!(Solution::combination_sum2(candidates, target), res);
}
