#![allow(dead_code)]

/*

// 2049. Count Nodes With the Highest Score
// https://leetcode.com/problems/count-nodes-with-the-highest-score/
// https://leetcode.cn/problems/count-nodes-with-the-highest-score/
//
// Medium
//
// There is a binary tree rooted at 0 consisting of n nodes. The nodes are labeled from 0 to n - 1.
// You are given a 0-indexed integer array parents representing the tree, where parents[i] is the parent of node i.
// Since node 0 is the root, parents[0] == -1.
//
// Each node has a score. To find the score of a node, consider if the node and the edges connected to it were removed.
// The tree would become one or more non-empty subtrees. The size of a subtree is the number of the nodes in it.
// The score of the node is the product of the sizes of all those subtrees.

Return the number of nodes that have the highest score.

Example 1:
example-1

Input: parents = [-1,2,0,2,0]
Output: 3
Explanation:
- The score of node 0 is: 3 * 1 = 3
- The score of node 1 is: 4 = 4
- The score of node 2 is: 1 * 1 * 2 = 2
- The score of node 3 is: 4 = 4
- The score of node 4 is: 4 = 4
The highest score is 4, and three nodes (node 1, node 3, and node 4) have the highest score.

Example 2:
example-2

Input: parents = [-1,2,0]
Output: 2
Explanation:
- The score of node 0 is: 2 = 2
- The score of node 1 is: 2 = 2
- The score of node 2 is: 1 * 1 = 1
The highest score is 2, and two nodes (node 0 and node 1) have the highest score.

Constraints:

    n == parents.length
    2 <= n <= 10^5
    parents[0] == -1
    0 <= parents[i] <= n - 1 for i != 0
    parents represents a valid binary tree.
*/

struct Solution;

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        fn dfs(al: &Vec<Vec<i64>>, s: &mut Vec<i64>, i: i64) -> i64 {
            let mut prod = 1;
            let mut sum = 1;
            for &j in &al[i as usize] {
                let cnt = dfs(al, s, j);
                prod *= cnt;
                sum += cnt;
            }
            s[i as usize] = prod * (al.len() as i64 - sum).max(1);
            if i == 0 {
                let s_max = *s.iter().max().unwrap();
                s.iter().filter(|&&x| x == s_max).count() as i64
            } else {
                sum
            }
        }

        let mut al = vec![vec![]; parents.len()];
        let mut s = vec![0; parents.len()];
        for (i, &p) in parents.iter().enumerate().skip(1) {
            al[p as usize].push(i as i64);
        }
        dfs(&al, &mut s, 0) as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![-1, 2, 0, 2, 0], 3),
        (vec![-1, 2, 0], 2),
        (vec![-1, 0, 1, 0, 3, 3], 1),
        (vec![-1, 0, 1, 0, 3, 3, 1], 1),
    ];
    for (parents, expected) in cases {
        assert_eq!(Solution::count_highest_score_nodes(parents), expected);
    }
}
