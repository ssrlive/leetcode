#![allow(dead_code)]

// 3378. Count Connected Components in LCM Graph
// https://leetcode.com/problems/count-connected-components-in-lcm-graph/
// https://leetcode.cn/problems/count-connected-components-in-lcm-graph/
//
// Hard
//
// You are given an array of integers nums of size n and a positive integer threshold.
//
// There is a graph consisting of n nodes with the ith node having a value of nums[i].
// Two nodes i and j in the graph are connected via an undirected edge if lcm(nums[i], nums[j]) <= threshold.
//
// Return the number of connected components in this graph.
//
// A connected component is a subgraph of a graph in which there exists a path between any two vertices,
// and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
//
// The term lcm(a, b) denotes the least common multiple of a and b.
//
// Example 1:
//
// Input: nums = [2,4,8,3,9], threshold = 5
//
// Output: 4
//
// Explanation:
//
// The four connected components are (2, 4), (3), (8), (9).
//
// Example 2:
//
// Input: nums = [2,4,8,3,9,12], threshold = 10
//
// Output: 2
//
// Explanation:
//
// The two connected components are (2, 3, 4, 8, 9), and (12).
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^9
// All elements of nums are unique.
// 1 <= threshold <= 2 * 10^5
//

struct Solution;

impl Solution {
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len() as i32;
        let mut adj = std::collections::HashMap::new();
        let mut components = 0;
        for j in 0..n {
            let num = nums[j as usize];
            if num > threshold {
                components += 1;
            }
            let mut i = num;
            while i <= threshold {
                adj.entry(num).or_insert(vec![]).push(i);
                adj.entry(i).or_insert(vec![]).push(num);
                i += num;
            }
        }
        let mut vis = std::collections::HashMap::new();
        for (&i, _) in adj.iter() {
            let mut q = std::collections::VecDeque::new();
            if vis.contains_key(&i) {
                continue;
            }
            q.push_back(i);
            vis.insert(i, 1);
            components += 1;
            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for &v in adj.get(&u).unwrap() {
                    if let std::collections::hash_map::Entry::Vacant(e) = vis.entry(v) {
                        e.insert(1);
                        q.push_back(v);
                    }
                }
            }
        }
        components
    }
}

#[test]
fn test() {
    let nums = vec![2, 4, 8, 3, 9];
    let threshold = 5;
    let output = 4;
    assert_eq!(Solution::count_components(nums, threshold), output);

    let nums = vec![2, 4, 8, 3, 9, 12];
    let threshold = 10;
    let output = 2;
    assert_eq!(Solution::count_components(nums, threshold), output);
}
