#![allow(dead_code)]

// 839. Similar String Groups
// https://leetcode.com/problems/similar-string-groups/
// https://leetcode.cn/problems/similar-string-groups/
//
// Two strings X and Y are similar if we can swap two letters (in different positions) of X, so that it equals Y.
// Also two strings X and Y are similar if they are equal.
//
// For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar,
// but "star" is not similar to "tars", "rats", or "arts".
//
// Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.
// Notice that "tars" and "arts" are in the same group even though they are not similar.
// Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.
//
// We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?
//
// Example 1:
//
// Input: strs = ["tars","rats","arts","star"]
// Output: 2
//
// Example 2:
//
// Input: strs = ["omv","ovm"]
// Output: 1
//
// Constraints:
//
// - 1 <= strs.length <= 300
// - 1 <= strs[i].length <= 300
// - strs[i] consists of lowercase letters only.
// - All words in strs have the same length and are anagrams of each other.
//

struct Solution;

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        fn is_similar(s1: &str, s2: &str) -> Option<bool> {
            let mut diff = 0;
            for i in 0..s1.len() {
                if s1.chars().nth(i)? != s2.chars().nth(i)? {
                    diff += 1;
                    if diff > 2 {
                        return Some(false);
                    }
                }
            }
            Some(true)
        }

        let mut uf = UnionFind::new(strs.len());
        for i in 0..strs.len() {
            for j in i + 1..strs.len() {
                if is_similar(&strs[i], &strs[j]).unwrap_or_default() {
                    uf.union(i, j);
                }
            }
        }
        uf.count()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let size = vec![1; n];
        for (i, item) in parent.iter_mut().enumerate().take(n) {
            *item = i;
        }
        UnionFind { parent, size, count: n }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        self.count -= 1;
    }

    fn count(&self) -> i32 {
        self.count as i32
    }
}

#[test]
fn test() {
    let strs = vec!["tars", "rats", "arts", "star"];
    let strs = strs.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::num_similar_groups(strs), 2);
}
