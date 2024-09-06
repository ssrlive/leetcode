#![allow(dead_code)]

// 3277. Maximum XOR Score Subarray Queries
// https://leetcode.com/problems/maximum-xor-score-subarray-queries/
// https://leetcode.cn/problems/maximum-xor-score-subarray-queries/
//
// Hard
//
// You are given an array nums of n integers, and a 2D integer array queries of size q, where queries[i] = [li, ri].
//
// For each query, you must find the maximum XOR score of any subarray of nums[li..ri].
//
// The XOR score of an array a is found by repeatedly applying the following operations
// on a so that only one element remains, that is the score:
//
//     Simultaneously replace a[i] with a[i] XOR a[i + 1] for all indices i except the last one.
//     Remove the last element of a.
//
// Return an array answer of size q where answer[i] is the answer to query i.
//
// Example 1:
//
// Input: nums = [2,8,4,32,16,1], queries = [[0,2],[1,4],[0,5]]
//
// Output: [12,60,60]
//
// Explanation:
//
// In the first query, nums[0..2] has 6 subarrays [2], [8], [4], [2, 8], [8, 4],
// and [2, 8, 4] each with a respective XOR score of 2, 8, 4, 10, 12, and 6.
// The answer for the query is 12, the largest of all XOR scores.
//
// In the second query, the subarray of nums[1..4] with the largest XOR score is nums[1..4] with a score of 60.
//
// In the third query, the subarray of nums[0..5] with the largest XOR score is nums[1..4] with a score of 60.
//
// Example 2:
//
// Input: nums = [0,7,3,2,8,5,1], queries = [[0,3],[1,5],[2,4],[2,6],[5,6]]
//
// Output: [7,14,11,14,5]
//
// Explanation:
// Index	nums[li..ri]	Maximum XOR Score Subarray	Maximum Subarray XOR Score
// 0	[0, 7, 3, 2]	[7]	7
// 1	[7, 3, 2, 8, 5]	[7, 3, 2, 8]	14
// 2	[3, 2, 8]	[3, 2, 8]	11
// 3	[3, 2, 8, 5, 1]	[2, 8, 5, 1]	14
// 4	[5, 1]	[5]	5
//
// Constraints:
//
//     1 <= n == nums.length <= 2000
//     0 <= nums[i] <= 231 - 1
//     1 <= q == queries.length <= 105
//     queries[i].length == 2
//     queries[i] = [li, ri]
//     0 <= li <= ri <= n - 1
//

struct Solution;

impl Solution {
    pub fn maximum_subarray_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = nums[i];
        }

        for len in 2..=n {
            for st in 0..n - len + 1 {
                let end = st + len - 1;
                dp[st][end] = dp[st][end - 1] ^ dp[st + 1][end];
            }
        }

        for len in 2..=n {
            for st in 0..n - len + 1 {
                let end = st + len - 1;
                dp[st][end] = dp[st][end].max(dp[st][end - 1]).max(dp[st + 1][end]);
            }
        }

        let mut res = vec![];
        for quer in queries {
            let u = quer[0] as usize;
            let v = quer[1] as usize;
            res.push(dp[u][v]);
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 8, 4, 32, 16, 1];
    let queries = vec![vec![0, 2], vec![1, 4], vec![0, 5]];
    let res = vec![12, 60, 60];
    assert_eq!(Solution::maximum_subarray_xor(nums, queries), res);

    let nums = vec![0, 7, 3, 2, 8, 5, 1];
    let queries = vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]];
    let res = vec![7, 14, 11, 14, 5];
    assert_eq!(Solution::maximum_subarray_xor(nums, queries), res);
}
