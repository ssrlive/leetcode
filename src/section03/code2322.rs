#![allow(dead_code)]

/*

// 2322. Minimum Score After Removals on a Tree
// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/
// https://leetcode.cn/problems/minimum-score-after-removals-on-a-tree/
//
// Hard
//
// There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.

You are given a 0-indexed integer array nums of length n where nums[i] represents the value of the ith node. You are also given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.

Remove two distinct edges of the tree to form three connected components. For a pair of removed edges, the following steps are defined:

    Get the XOR of all the values of the nodes for each of the three components respectively.
    The difference between the largest XOR value and the smallest XOR value is the score of the pair.

    For example, say the three components have the node values: [4,5,7], [1,9], and [3,3,3]. The three XOR values are 4 ^ 5 ^ 7 = 6, 1 ^ 9 = 8, and 3 ^ 3 ^ 3 = 3. The largest XOR value is 8 and the smallest XOR value is 3. The score is then 8 - 3 = 5.

Return the minimum score of any possible pair of edge removals on the given tree.

Example 1:

Input: nums = [1,5,5,4,11], edges = [[0,1],[1,2],[1,3],[3,4]]
Output: 9
Explanation: The diagram above shows a way to make a pair of removals.
- The 1st component has nodes [1,3,4] with values [5,4,11]. Its XOR value is 5 ^ 4 ^ 11 = 10.
- The 2nd component has node [0] with value [1]. Its XOR value is 1 = 1.
- The 3rd component has node [2] with value [5]. Its XOR value is 5 = 5.
The score is the difference between the largest and smallest XOR value which is 10 - 1 = 9.
It can be shown that no other pair of removals will obtain a smaller score than 9.

Example 2:

Input: nums = [5,5,2,4,4,2], edges = [[0,1],[1,2],[5,2],[4,3],[1,3]]
Output: 0
Explanation: The diagram above shows a way to make a pair of removals.
- The 1st component has nodes [3,4] with values [4,4]. Its XOR value is 4 ^ 4 = 0.
- The 2nd component has nodes [1,0] with values [5,5]. Its XOR value is 5 ^ 5 = 0.
- The 3rd component has nodes [2,5] with values [2,2]. Its XOR value is 2 ^ 2 = 0.
The score is the difference between the largest and smallest XOR value which is 0 - 0 = 0.
We cannot obtain a smaller score than 0.

Constraints:

    n == nums.length
    3 <= n <= 1000
    1 <= nums[i] <= 10^8
    edges.length == n - 1
    edges[i].length == 2
    0 <= ai, bi < n
    ai != bi
    edges represents a valid tree.
*/

struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(dp: &mut Vec<i32>, last: &mut Vec<i32>, nums: &Vec<i32>, al: &Vec<Vec<i32>>, i: usize, p: i32, ids: &mut i32) -> i32 {
            let mut res = nums[i];
            for j in al[i].iter() {
                if *j != p {
                    let id = *ids;
                    *ids += 1;
                    dp[id as usize] = dfs(dp, last, nums, al, *j as usize, i as i32, ids);
                    last[id as usize] = *ids;
                    res ^= dp[id as usize];
                }
            }
            res
        }

        let mut dp = vec![0; 1000];
        let mut last = vec![0; 1000];
        let mut ids = 0;
        let mut res = i32::MAX;
        let mut al = vec![vec![]; nums.len()];
        for e in edges.iter() {
            al[e[0] as usize].push(e[1]);
            al[e[1] as usize].push(e[0]);
        }
        let all = dfs(&mut dp, &mut last, &nums, &al, 0, -1, &mut ids);
        for i in 0..edges.len() {
            for j in i + 1..edges.len() {
                let p1 = if j < last[i] as usize { all ^ dp[i] } else { all ^ dp[i] ^ dp[j] };
                let p2 = if j < last[i] as usize { dp[i] ^ dp[j] } else { dp[i] };
                let arr = [p1, p2, dp[j]];
                let max = arr.iter().max().unwrap();
                let min = arr.iter().min().unwrap();
                res = res.min(max - min);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 5, 5, 4, 11], vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]], 9),
        (
            vec![5, 5, 2, 4, 4, 2],
            vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]],
            0,
        ),
    ];
    for (nums, edges, expected) in cases {
        assert_eq!(Solution::minimum_score(nums, edges), expected);
    }
}
