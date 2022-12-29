#![allow(dead_code)]

// 851. Loud and Rich
// https://leetcode.com/problems/loud-and-rich/
// https://leetcode.cn/problems/loud-and-rich/
//
// There is a group of n people labeled from 0 to n - 1 where each person has a different amount of money and a different level of quietness.
//
// You are given an array richer where richer[i] = [ai, bi] indicates that ai has more money than bi and an integer array quiet where quiet[i]
// is the quietness of the ith person. All the given data in richer are logically correct
// (i.e., the data will not lead you to a situation where x is richer than y and y is richer than x at the same time).
//
// Return an integer array answer where answer[x] = y if y is the least quiet person
// (that is, the person y with the smallest value of quiet[y]) among all people who definitely have equal to or more money than the person x.
//
// Example 1:
//
// Input: richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]], quiet = [3,2,5,4,6,1,7,0]
// Output: [5,5,2,5,4,5,6,7]
// Explanation:
// answer[0] = 5.
// Person 5 has more money than 3, which has more money than 1, which has more money than 0.
// The only person who is quieter (has lower quiet[x]) is person 7, but it is not clear if they have more money than person 0.
// answer[7] = 7.
// Among all people that definitely have equal to or more money than person 7 (which could be persons 3, 4, 5, 6, or 7), the person who is the quietest (has lower quiet[x]) is person 7.
// The other answers can be filled out with similar reasoning.
//
// Example 2:
//
// Input: richer = [], quiet = [0]
// Output: [0]
//
// Constraints:
//
// - n == quiet.length
// - 1 <= n <= 500
// - 0 <= quiet[i] < n
// - All the values of quiet are unique.
// - 0 <= richer.length <= n * (n - 1) / 2
// - 0 <= ai, bi < n
// - ai != bi
// - All the pairs of richer are unique.
// - The observations in richer are all logically consistent.
//

struct Solution;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        fn dfs(adj: &Vec<Vec<usize>>, visit: &mut Vec<bool>, quiet: &Vec<i32>, node: usize, ind: &mut usize) {
            visit[node] = true;
            if quiet[node] < quiet[*ind] {
                *ind = node;
            }
            for &i in &adj[node] {
                if !visit[i] {
                    dfs(adj, visit, quiet, i, ind);
                }
            }
        }

        let n = quiet.len();
        let mut adj = vec![vec![]; n];
        for v in richer {
            adj[v[1] as usize].push(v[0] as usize);
        }
        let mut ans = vec![0; n];
        for (i, item) in ans.iter_mut().enumerate().take(n) {
            let mut visit = vec![false; n];
            let mut ind = i;
            dfs(&adj, &mut visit, &quiet, i, &mut ind);
            *item = ind as i32;
        }
        ans
    }
}

#[test]
fn test() {
    let richer = vec![
        vec![1, 0],
        vec![2, 1],
        vec![3, 1],
        vec![3, 7],
        vec![4, 3],
        vec![5, 3],
        vec![6, 3],
    ];
    let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
    let ans = vec![5, 5, 2, 5, 4, 5, 6, 7];
    assert_eq!(Solution::loud_and_rich(richer, quiet), ans);

    let richer = vec![];
    let quiet = vec![0];
    let ans = vec![0];
    assert_eq!(Solution::loud_and_rich(richer, quiet), ans);
}
