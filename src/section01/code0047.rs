#![allow(dead_code)]

// 47. Permutations II
// https://leetcode.com/problems/permutations-ii/
// https://leetcode.cn/problems/permutations-ii/
//
// Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
//
// Example 1:
//
// Input: nums = [1,1,2]
// Output:
// [[1,1,2],
//  [1,2,1],
//  [2,1,1]]
//
// Example 2:
//
// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// Constraints:
//
// - 1 <= nums.length <= 8
// - -10 <= nums[i] <= 10
//

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>) {
            if tmp.len() == nums.len() {
                res.push(tmp.clone());
                return;
            }
            for i in 0..nums.len() {
                let n = nums[i];
                if visited[i] {
                    continue;
                }
                if i > 0 && n == nums[i - 1] && !visited[i - 1] {
                    continue;
                }
                tmp.push(n);
                visited[i] = true;
                dfs(nums, tmp, res, visited);
                tmp.pop();
                visited[i] = false;
            }
        }
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        dfs(&nums, &mut vec![], &mut res, &mut vec![false; nums.len()]);
        res
    }
}

#[test]
fn test() {
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    assert_eq!(Solution::permute_unique(vec![1, 1, 2]), expected);

    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    assert_eq!(Solution::permute_unique(vec![1, 2, 3]), expected);
}
