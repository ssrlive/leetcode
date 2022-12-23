#![allow(dead_code)]

// 39. Combination Sum
// https://leetcode.com/problems/combination-sum/
// https://leetcode.cn/problems/combination-sum/
//
// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations
// of candidates where the chosen numbers sum to target. You may return the combinations in any order.
//
// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
// frequency
//  of at least one of the chosen numbers is different.
//
// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
//
// Example 1:
//
// Input: candidates = [2,3,6,7], target = 7
// Output: [[2,2,3],[7]]
// Explanation:
// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations.
//
// Example 2:
//
// Input: candidates = [2,3,5], target = 8
// Output: [[2,2,2,2],[2,3,3],[3,5]]
//
// Example 3:
//
// Input: candidates = [2], target = 1
// Output: []
//
// Constraints:
//
// - 1 <= candidates.length <= 30
// - 2 <= candidates[i] <= 40
// - All elements of candidates are distinct.
// - 1 <= target <= 40
//

struct Solution;

impl Solution {
    pub fn combination_sum(nums: Vec<i32>, t: i32) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, t: i32, len: usize, i: usize) {
            if t < 0 || i >= len {
                return;
            } else if t == 0 {
                res.push(temp.clone());
                return;
            }
            let x = nums[i];
            temp.push(x);
            dfs(nums, res, temp, t - x, len, i);
            temp.pop();
            dfs(nums, res, temp, t, len, i + 1);
        }

        let len = nums.len();
        let mut res = Vec::with_capacity(100);
        let mut temp = Vec::with_capacity(100);
        dfs(&nums, &mut res, &mut temp, t, len, 0);
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 6, 7];
    let res = Solution::combination_sum(nums, 7);
    assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);

    let nums = vec![2, 3, 5];
    let res = Solution::combination_sum(nums, 8);
    assert_eq!(res, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);

    let nums = vec![2];
    let res = Solution::combination_sum(nums, 1);
    assert_eq!(res, Vec::<Vec<i32>>::new());
}
