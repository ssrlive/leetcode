#![allow(dead_code)]

/*

// 1697. Checking Existence of Edge Length Limited Paths
// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/
// https://leetcode.cn/problems/checking-existence-of-edge-length-limited-paths/
//
// Hard
//
// An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between two nodes.

Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each queries[j] whether there is a path between pj and qj such that each edge on the path has a distance strictly less than limitj .

Return a boolean array answer, where answer.length == queries.length and the jth value of answer is true if there is a path for queries[j] is true, and false otherwise.

Example 1:

Input: n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
Output: [false,true]
Explanation: The above figure shows the given graph. Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
For the first query, between 0 and 1 there is no path where each distance is less than 2, thus we return false for this query.
For the second query, there is a path (0 -> 1 -> 2) of two edges with distances less than 5, thus we return true for this query.

Example 2:

Input: n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
Output: [true,false]
Exaplanation: The above figure shows the given graph.

Constraints:

    2 <= n <= 10^5
    1 <= edgeList.length, queries.length <= 10^5
    edgeList[i].length == 3
    queries[j].length == 3
    0 <= ui, vi, pj, qj <= n - 1
    ui != vi
    pj != qj
    1 <= disi, limitj <= 10^9
    There may be multiple edges between two nodes.
*/

struct Solution;

impl Solution {
    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut edge_list = edge_list;
        let mut queries = queries;
        let mut ans = vec![false; queries.len()];
        let mut uf = UnionFind::new(n as usize);
        edge_list.sort_by(|a, b| a[2].cmp(&b[2]));
        queries.iter_mut().enumerate().for_each(|(i, q)| q.push(i as i32));
        queries.sort_by(|a, b| a[2].cmp(&b[2]));
        let mut j = 0;
        for q in queries {
            let limit = q[2];
            while j < edge_list.len() && edge_list[j][2] < limit {
                uf.union(edge_list[j][0] as usize, edge_list[j][1] as usize);
                j += 1;
            }
            ans[q[3] as usize] = uf.find(q[0] as usize) == uf.find(q[1] as usize);
        }
        ans
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let rank = vec![0; n];
        for (i, item) in parent.iter_mut().enumerate() {
            *item = i;
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        match self.rank[x].cmp(&self.rank[y]) {
            std::cmp::Ordering::Less => self.parent[x] = y,
            std::cmp::Ordering::Greater => self.parent[y] = x,
            std::cmp::Ordering::Equal => {
                self.parent[y] = x;
                self.rank[x] += 1;
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            3,
            vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
            vec![vec![0, 1, 2], vec![0, 2, 5]],
            vec![false, true],
        ),
        (
            5,
            vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
            vec![vec![0, 4, 14], vec![1, 4, 13]],
            vec![true, false],
        ),
    ];
    for (n, ed, qu, ex) in cases {
        assert_eq!(Solution::distance_limited_paths_exist(n, ed, qu), ex);
    }
}
