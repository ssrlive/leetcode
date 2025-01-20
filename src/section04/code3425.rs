#![allow(dead_code)]

// 3425. Longest Special Path
// https://leetcode.com/problems/longest-special-path/
// https://leetcode.cn/problems/longest-special-path/
//
// Hard
//
// You are given an undirected tree rooted at node 0 with n nodes numbered from 0 to n - 1, represented by a 2D array edges
// of length n - 1, where edges[i] = [ui, vi, lengthi] indicates an edge between nodes ui and vi with length lengthi.
// You are also given an integer array nums, where nums[i] represents the value at node i.
//
// A special path is defined as a downward path from an ancestor node to a descendant node such that all the values of the nodes in that path are unique.
//
// Note that a path may start and end at the same node.
//
// Return an array result of size 2, where result[0] is the length of the longest special path,
// and result[1] is the minimum number of nodes in all possible longest special paths.
//
// Example 1:
//
// Input: edges = [[0,1,2],[1,2,3],[1,3,5],[1,4,4],[2,5,6]], nums = [2,1,2,1,3,1]
//
// Output: [6,2]
//
// Explanation:
// In the image below, nodes are colored by their corresponding values in nums
//
// The longest special paths are 2 -> 5 and 0 -> 1 -> 4, both having a length of 6. The minimum number of nodes across all longest special paths is 2.
//
// Example 2:
//
// Input: edges = [[1,0,8]], nums = [2,2]
//
// Output: [0,1]
//
// Explanation:
//
// The longest special paths are 0 and 1, both having a length of 0. The minimum number of nodes across all longest special paths is 1.
//
// Constraints:
//
//     2 <= n <= 5 * 10^4
//     edges.length == n - 1
//     edges[i].length == 3
//     0 <= ui, vi < n
//     1 <= lengthi <= 10^3
//     nums.length == n
//     0 <= nums[i] <= 5 * 10^4
//     The input is generated such that edges represents a valid tree.
//

struct Solution;

impl Solution {
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            i: usize,
            p: i32,
            al: &[Vec<[i32; 2]>],
            nums: &[i32],
            left: i32,
            cur_depth: i32,
            ps: &mut Vec<i32>,
            depth: &mut [i32],
            res: &mut (i32, i32),
        ) {
            let prev_depth = std::mem::replace(&mut depth[nums[i] as usize], cur_depth);
            let left = left.max(prev_depth);
            *res = std::cmp::min(*res, (-(ps[ps.len() - 1] - ps[left as usize]), cur_depth - left));
            for &[j, l] in &al[i] {
                if j != p {
                    ps.push(ps[ps.len() - 1] + l);
                    dfs(j as usize, i as i32, al, nums, left, cur_depth + 1, ps, depth, res);
                    ps.pop();
                }
            }
            depth[nums[i] as usize] = prev_depth;
        }

        let n = nums.len();
        let mut al = vec![vec![]; n];
        for e in &edges {
            al[e[0] as usize].push([e[1], e[2]]);
            al[e[1] as usize].push([e[0], e[2]]);
        }
        let mut res = (0, 1);
        let mut depth = vec![0; 50001];
        dfs(0, -1, &al, &nums, 0, 1, &mut vec![0], &mut depth, &mut res);
        vec![-res.0, res.1]
    }
}

#[test]
fn test() {
    let edges = vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]];
    let nums = vec![2, 1, 2, 1, 3, 1];
    let res = vec![6, 2];
    assert_eq!(Solution::longest_special_path(edges, nums), res);

    let edges = vec![vec![1, 0, 8]];
    let nums = vec![2, 2];
    let res = vec![0, 1];
    assert_eq!(Solution::longest_special_path(edges, nums), res);
}
