#![allow(dead_code)]

// 1202. Smallest String With Swaps
// https://leetcode.com/problems/smallest-string-with-swaps/
// https://leetcode.cn/problems/smallest-string-with-swaps/
//
// Medium
//
// You are given a string s, and an array of pairs of indices in the string pairs where pairs[i] = [a, b] indicates 2 indices(0-indexed) of the string.
//
// You can swap the characters at any pair of indices in the given pairs any number of times.
//
// Return the lexicographically smallest string that s can be changed to after using the swaps.
//
// Example 1:
//
// Input: s = "dcab", pairs = [[0,3],[1,2]]
// Output: "bacd"
// Explaination:
// Swap s[0] and s[3], s = "bcad"
// Swap s[1] and s[2], s = "bacd"
//
// Example 2:
//
// Input: s = "dcab", pairs = [[0,3],[1,2],[0,2]]
// Output: "abcd"
// Explaination:
// Swap s[0] and s[3], s = "bcad"
// Swap s[0] and s[2], s = "acbd"
// Swap s[1] and s[2], s = "abcd"
//
// Example 3:
//
// Input: s = "cba", pairs = [[0,1],[1,2]]
// Output: "abc"
// Explaination:
// Swap s[0] and s[1], s = "bca"
// Swap s[1] and s[2], s = "bac"
// Swap s[0] and s[1], s = "abc"
//
// Constraints:
//
// -    1 <= s.length <= 10^5
// -    0 <= pairs.length <= 10^5
// -    0 <= pairs[i][0], pairs[i][1] < s.length
// -    s only contains lower case English letters.
//

struct Solution;

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut uf = UnionFind::new(s.len());
        for pair in pairs {
            uf.union(pair[0] as usize, pair[1] as usize);
        }
        let mut map = std::collections::HashMap::new();
        for i in 0..s.len() {
            let root = uf.find(i);
            map.entry(root).or_insert_with(Vec::new).push(i);
        }
        for (_, v) in map {
            let mut chars = v.iter().map(|&i| s[i]).collect::<Vec<_>>();
            chars.sort();
            for (i, &c) in v.iter().enumerate() {
                s[c] = chars[i];
            }
        }
        String::from_utf8(s).unwrap()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut rank = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            rank.push(0);
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
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => self.parent[x_root] = y_root,
            std::cmp::Ordering::Greater => self.parent[y_root] = x_root,
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        ("dcab", vec![vec![0, 3], vec![1, 2]], "bacd"),
        ("dcab", vec![vec![0, 3], vec![1, 2], vec![0, 2]], "abcd"),
        ("cba", vec![vec![0, 1], vec![1, 2]], "abc"),
    ];
    for (s, pairs, expected) in cases {
        assert_eq!(Solution::smallest_string_with_swaps(s.to_string(), pairs), expected);
    }
}
