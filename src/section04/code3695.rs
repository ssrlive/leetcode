#![allow(dead_code)]

// 3695. Maximize Alternating Sum Using Swaps
// https://leetcode.com/problems/maximize-alternating-sum-using-swaps/
// https://leetcode.cn/problems/maximize-alternating-sum-using-swaps/
//
// Hard
//
// You are given an integer array nums.
//
// You want to maximize the alternating sum of nums, which is defined as the value obtained by adding elements at even indices and subtracting elements at odd indices. That is, nums[0] - nums[1] + nums[2] - nums[3]...
//
// You are also given a 2D integer array swaps where swaps[i] = [pi, qi]. For each pair [pi, qi] in swaps, you are allowed to swap the elements at indices pi and qi. These swaps can be performed any number of times and in any order.
//
// Return the maximum possible alternating sum of nums.
//
// Example 1:
//
// Input: nums = [1,2,3], swaps = [[0,2],[1,2]]
//
// Output: 4
//
// Explanation:
//
// The maximum alternating sum is achieved when nums is [2, 1, 3] or [3, 1, 2]. As an example, you can obtain nums = [2, 1, 3] as follows.
//
// Swap nums[0] and nums[2]. nums is now [3, 2, 1].
// Swap nums[1] and nums[2]. nums is now [3, 1, 2].
// Swap nums[0] and nums[2]. nums is now [2, 1, 3].
//
// Example 2:
//
// Input: nums = [1,2,3], swaps = [[1,2]]
//
// Output: 2
//
// Explanation:
//
// The maximum alternating sum is achieved by not performing any swaps.
//
// Example 3:
//
// Input: nums = [1,1000000000,1,1000000000,1,1000000000], swaps = []
//
// Output: -2999999997
//
// Explanation:
//
// Since we cannot perform any swaps, the maximum alternating sum is achieved by not performing any swaps.
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
// 0 <= swaps.length <= 10^5
// swaps[i] = [pi, qi]
// 0 <= pi < qi <= nums.length - 1
// [pi, qi] != [pj, qj]
//

struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>, swaps: Vec<Vec<i32>>) -> i64 {
        fn dfs(node: usize, t: usize, tree: &mut std::collections::HashMap<usize, Vec<usize>>, vis: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            tree.entry(t).or_default().push(node);
            vis[node] = true;
            for &it in &adj[node] {
                if !vis[it] {
                    dfs(it, t, tree, vis, adj);
                }
            }
        }

        let n = nums.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for swap in swaps {
            let u = swap[0] as usize;
            let v = swap[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut tree: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
        let mut vis: Vec<bool> = vec![false; n];
        let mut cnt = 0;
        for i in 0..n {
            if !vis[i] {
                dfs(i, cnt, &mut tree, &mut vis, &adj);
                cnt += 1;
            }
        }

        let mut ans: i64 = 0;
        for i in 0..cnt {
            let mut val: Vec<i64> = vec![];
            if let Some(indices) = tree.get(&i) {
                for &idx in indices {
                    val.push(nums[idx] as i64);
                }
            }
            val.sort_unstable();

            let mut l = 0;
            let mut r = val.len() as isize - 1;
            if let Some(indices) = tree.get(&i) {
                for &idx in indices {
                    if idx % 2 == 1 {
                        ans -= val[l];
                        l += 1;
                    } else {
                        ans += val[r as usize];
                        r -= 1;
                    }
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 2, 3];
    let swaps1 = vec![vec![0, 2], vec![1, 2]];
    assert_eq!(Solution::max_alternating_sum(nums1, swaps1), 4);

    let nums2 = vec![1, 2, 3];
    let swaps2 = vec![vec![1, 2]];
    assert_eq!(Solution::max_alternating_sum(nums2, swaps2), 2);

    let nums3 = vec![1, 1000000000, 1, 1000000000, 1, 1000000000];
    let swaps3: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::max_alternating_sum(nums3, swaps3), -2999999997);
}
