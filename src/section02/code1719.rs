#![allow(dead_code)]

/*

// 1719. Number Of Ways To Reconstruct A Tree
// https://leetcode.com/problems/number-of-ways-to-reconstruct-a-tree/
// https://leetcode.cn/problems/number-of-ways-to-reconstruct-a-tree/
//
// Hard
//
// You are given an array pairs, where pairs[i] = [xi, yi], and:

    There are no duplicates.
    xi < yi

Let ways be the number of rooted trees that satisfy the following conditions:

    The tree consists of nodes whose values appeared in pairs.
    A pair [xi, yi] exists in pairs if and only if xi is an ancestor of yi or yi is an ancestor of xi.
    Note: the tree does not have to be a binary tree.

Two ways are considered to be different if there is at least one node that has different parents in both ways.

Return:

    0 if ways == 0
    1 if ways == 1
    2 if ways > 1

A rooted tree is a tree that has a single root node, and all edges are oriented to be outgoing from the root.

An ancestor of a node is any node on the path from the root to that node (excluding the node itself). The root has no ancestors.

Example 1:

Input: pairs = [[1,2],[2,3]]
Output: 1
Explanation: There is exactly one valid rooted tree, which is shown in the above figure.

Example 2:

Input: pairs = [[1,2],[2,3],[1,3]]
Output: 2
Explanation: There are multiple valid rooted trees. Three of them are shown in the above figures.

Example 3:

Input: pairs = [[1,2],[2,3],[2,4],[1,5]]
Output: 0
Explanation: There are no valid rooted trees.

Constraints:

    1 <= pairs.length <= 10^5
    1 <= xi < yi <= 500
    The elements in pairs are unique.
*/

struct Solution;

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        // BinaryHeap act as priority_queue
        use std::collections::{BinaryHeap, HashMap, HashSet};
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for pair in pairs.iter() {
            adj.entry(pair[0]).or_default().insert(pair[1]);
            adj.entry(pair[1]).or_default().insert(pair[0]);
        }
        let mut q: BinaryHeap<(usize, i32)> = BinaryHeap::new();
        for (x, arr) in adj.iter() {
            q.push((arr.len(), *x));
        }
        let n = q.len();
        let mut multiple = false;
        let mut seen: HashSet<i32> = HashSet::new();
        while let Some((sz, v)) = q.pop() {
            let mut u = 0;
            let mut usz = n + 1;
            if !seen.is_empty() {
                for x in adj.get(&v).unwrap() {
                    if adj.get(x).unwrap().len() < usz && seen.contains(x) {
                        u = *x;
                        usz = adj.get(x).unwrap().len();
                    }
                }
            }
            seen.insert(v);
            if u == 0 {
                if sz != (n - 1) {
                    return 0;
                }
                continue;
            }
            for x in adj.get(&v).unwrap() {
                if *x == u {
                    continue;
                }
                if !adj.get(&u).unwrap().contains(x) {
                    return 0;
                }
            }
            if usz == sz {
                multiple = true;
            }
        }
        if multiple {
            2
        } else {
            1
        }
    }
}

#[test]
fn test() {
    let pairs = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(Solution::check_ways(pairs), 1);
    let pairs = vec![vec![1, 2], vec![2, 3], vec![1, 3]];
    assert_eq!(Solution::check_ways(pairs), 2);
    let pairs = vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]];
    assert_eq!(Solution::check_ways(pairs), 0);
}
