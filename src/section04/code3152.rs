#![allow(dead_code)]

// 3152. Special Array II
// https://leetcode.com/problems/special-array-ii/
// https://leetcode.cn/problems/special-array-ii/
//
// Medium
//
// An array is considered special if every pair of its adjacent elements contains two numbers with different parity.
//
// You are given an array of integer nums and a 2D integer matrix queries, where for queries[i] = [fromi, toi]
// your task is to check that subarray nums[fromi..toi] is special or not.
//
// Return an array of booleans answer such that answer[i] is true if nums[fromi..toi] is special.
//
// Example 1:
//
// Input: nums = [3,4,1,2,6], queries = [[0,4]]
//
// Output: [false]
//
// Explanation:
//
// The subarray is [3,4,1,2,6]. 2 and 6 are both even.
//
// Example 2:
//
// Input: nums = [4,3,1,6], queries = [[0,2],[2,3]]
//
// Output: [false,true]
//
// Explanation:
//
// The subarray is [4,3,1]. 3 and 1 are both odd. So the answer to this query is false.
// The subarray is [1,6]. There is only one pair: (1,6) and it contains numbers with different parity. So the answer to this query is true.
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^5
// 1 <= queries.length <= 10^5
// queries[i].length == 2
// 0 <= queries[i][0] <= queries[i][1] <= nums.length - 1
//

struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = Vec::new();
        let mut converted = vec![0];
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                j += 1;
            }
            converted.push(j);
        }
        for q in queries {
            ans.push(converted[q[0] as usize] == converted[q[1] as usize]);
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![3, 4, 1, 2, 6];
    let queries = vec![vec![0, 4]];
    let output = vec![false];
    assert_eq!(Solution::is_array_special(nums, queries), output);

    let nums = vec![4, 3, 1, 6];
    let queries = vec![vec![0, 2], vec![2, 3]];
    let output = vec![false, true];
    assert_eq!(Solution::is_array_special(nums, queries), output);
}
