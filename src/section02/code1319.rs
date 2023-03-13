#![allow(dead_code)]

// 1319. Number of Operations to Make Network Connected
// https://leetcode.com/problems/number-of-operations-to-make-network-connected/
// https://leetcode.cn/problems/number-of-operations-to-make-network-connected/
//
// Medium
//
// There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
//
// You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
//
// Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
//
// Example 1:
//
// Input: n = 4, connections = [[0,1],[0,2],[1,2]]
// Output: 1
// Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.
//
// Example 2:
//
// Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
// Output: 2
//
// Example 3:
//
// Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
// Output: -1
// Explanation: There are not enough cables.
//
// Constraints:
//
// -    1 <= n <= 105
// -    1 <= connections.length <= min(n * (n - 1) / 2, 10^5)
// -    connections[i].length == 2
// -    0 <= ai, bi < n
// -    ai != bi
// -    There are no repeated connections.
// -    No two computers are connected by more than one cable.
//

struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        let mut extra = 0;
        for c in connections {
            if !uf.union(c[0] as usize, c[1] as usize) {
                extra += 1;
            }
        }
        let mut count = 0;
        for i in 0..n {
            if uf.find(i) == i {
                count += 1;
            }
        }
        if count - 1 <= extra {
            count - 1
        } else {
            -1
        }
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        true
    }
}

#[test]
fn test() {
    let n = 4;
    let connections = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    assert_eq!(Solution::make_connected(n, connections), 1);

    let n = 6;
    let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]];
    assert_eq!(Solution::make_connected(n, connections), 2);

    let n = 6;
    let connections = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]];
    assert_eq!(Solution::make_connected(n, connections), -1);
}
