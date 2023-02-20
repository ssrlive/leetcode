#![allow(dead_code)]

/*

// 1766. Tree of Coprimes
// https://leetcode.com/problems/tree-of-coprimes/
// https://leetcode.cn/problems/tree-of-coprimes/
//
// Hard
//
// There is a tree (i.e., a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. Each node has a value associated with it, and the root of the tree is node 0.

To represent this tree, you are given an integer array nums and a 2D array edges. Each nums[i] represents the ith node's value, and each edges[j] = [uj, vj] represents an edge between nodes uj and vj in the tree.

Two values x and y are coprime if gcd(x, y) == 1 where gcd(x, y) is the greatest common divisor of x and y.

An ancestor of a node i is any other node on the shortest path from node i to the root. A node is not considered an ancestor of itself.

Return an array ans of size n, where ans[i] is the closest ancestor to node i such that nums[i] and nums[ans[i]] are coprime, or -1 if there is no such ancestor.

Example 1:

Input: nums = [2,3,3,2], edges = [[0,1],[1,2],[1,3]]
Output: [-1,0,0,1]
Explanation: In the above figure, each node's value is in parentheses.
- Node 0 has no coprime ancestors.
- Node 1 has only one ancestor, node 0. Their values are coprime (gcd(2,3) == 1).
- Node 2 has two ancestors, nodes 1 and 0. Node 1's value is not coprime (gcd(3,3) == 3), but node 0's
  value is (gcd(2,3) == 1), so node 0 is the closest valid ancestor.
- Node 3 has two ancestors, nodes 1 and 0. It is coprime with node 1 (gcd(3,2) == 1), so node 1 is its
  closest valid ancestor.

Example 2:

Input: nums = [5,6,10,2,3,6,15], edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]]
Output: [-1,0,-1,0,0,0,-1]

Constraints:

    nums.length == n
    1 <= nums[i] <= 50
    1 <= n <= 10^5
    edges.length == n - 1
    edges[j].length == 2
    0 <= uj, vj < n
    uj != vj
*/

struct Solution;

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        #[allow(clippy::too_many_arguments)]
        fn traverse(
            nums: &Vec<i32>,
            al: &Vec<Vec<i32>>,
            i: i32,
            parent: i32,
            level: i32,
            res: &mut Vec<i32>,
            cops: &mut HashMap<i32, Vec<i32>>,
            ancestors: &mut HashMap<i32, Vec<(i32, i32)>>,
        ) {
            let mut max_level = -1;
            for &cop in cops.entry(nums[i as usize]).or_insert(vec![]).iter() {
                let v = ancestors.entry(cop).or_insert(vec![]);
                if !v.is_empty() && v.last().unwrap().0 > max_level {
                    max_level = v.last().unwrap().0;
                    res[i as usize] = v.last().unwrap().1;
                }
            }
            ancestors.entry(nums[i as usize]).or_insert(vec![]).push((level, i));
            for &child in al[i as usize].iter() {
                if child != parent {
                    traverse(nums, al, child, i, level + 1, res, cops, ancestors);
                }
            }
            ancestors.get_mut(&nums[i as usize]).unwrap().pop();
        }

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        use std::collections::{HashMap, HashSet};
        let mut cops = HashMap::new();
        let mut ancestors = HashMap::new();
        let mut s = HashSet::new();
        for &n1 in nums.iter() {
            s.insert(n1);
        }
        for &n1 in s.iter() {
            for &n2 in s.iter() {
                if gcd(n1, n2) == 1 {
                    cops.entry(n1).or_insert(vec![]).push(n2);
                }
            }
        }
        let mut al = vec![vec![]; nums.len()];
        let mut res = vec![-1; nums.len()];
        for e in edges.iter() {
            al[e[0] as usize].push(e[1]);
            al[e[1] as usize].push(e[0]);
        }
        traverse(&nums, &al, 0, 0, 0, &mut res, &mut cops, &mut ancestors);
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![2, 3, 3, 2],
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![-1, 0, 0, 1],
        ),
        (
            vec![5, 6, 10, 2, 3, 6, 15],
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
            vec![-1, 0, -1, 0, 0, 0, -1],
        ),
        (
            vec![6, 2, 3, 6],
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            vec![-1, -1, 1, -1],
        ),
    ];
    for (nums, edges, expect) in cases {
        assert_eq!(Solution::get_coprimes(nums, edges), expect);
    }
}
