#![allow(dead_code)]

// 1361. Validate Binary Tree Nodes
// https://leetcode.com/problems/validate-binary-tree-nodes/
// https://leetcode.cn/problems/validate-binary-tree-nodes/
//
// Medium
//
// You have n binary tree nodes numbered from 0 to n - 1 where node i has two children leftChild[i] and rightChild[i],
// return true if and only if all the given nodes form exactly one valid binary tree.
//
// If node i has no left child then leftChild[i] will equal -1, similarly for the right child.
//
// Note that the nodes have no values and that we only use the node numbers in this problem.
//
// Example 1:
//
// Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,-1,-1,-1]
// Output: true
//
// Example 2:
//
// Input: n = 4, leftChild = [1,-1,3,-1], rightChild = [2,3,-1,-1]
// Output: false
//
// Example 3:
//
// Input: n = 2, leftChild = [1,0], rightChild = [-1,-1]
// Output: false
//
// Constraints:
//
// -    n == leftChild.length == rightChild.length
// -    1 <= n <= 10^4
// -    -1 <= leftChild[i], rightChild[i] <= n - 1
//

struct Solution;

impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut is_root = vec![true; n as usize];
        for v in 0..(n as usize) {
            if left_child[v] != -1 {
                is_root[left_child[v] as usize] = false;
            }
            if right_child[v] != -1 {
                is_root[right_child[v] as usize] = false;
            }
        }
        let root = is_root.iter().position(|&r| r);
        if root.is_none() {
            return false;
        }
        let root = root.unwrap() as i32;

        fn helper(cur: i32, l: &Vec<i32>, r: &Vec<i32>, visited: &mut Vec<bool>) -> bool {
            if cur == -1 {
                return true;
            }
            if visited[cur as usize] {
                return false;
            }
            visited[cur as usize] = true;
            helper(l[cur as usize], l, r, visited) && helper(r[cur as usize], l, r, visited)
        }

        let mut vis = vec![false; n as usize];
        helper(root, &left_child, &right_child, &mut vis) && !vis.contains(&false)
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![1, -1, 3, -1], vec![2, -1, -1, -1], true),
        (4, vec![1, -1, 3, -1], vec![2, 3, -1, -1], false),
        (2, vec![1, 0], vec![-1, -1], false),
        (6, vec![1, -1, -1, 4, -1, -1], vec![2, -1, -1, 5, -1, -1], false),
    ];
    for (n, l, r, e) in cases {
        assert_eq!(Solution::validate_binary_tree_nodes(n, l, r), e);
    }
}
