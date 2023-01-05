#![allow(dead_code)]

// 952. Largest Component Size by Common Factor
// https://leetcode.com/problems/largest-component-size-by-common-factor/
// https://leetcode.cn/problems/largest-component-size-by-common-factor/
//
// You are given an integer array of unique positive integers nums. Consider the following graph:
//
// There are nums.length nodes, labeled nums[0] to nums[nums.length - 1],
// There is an undirected edge between nums[i] and nums[j] if nums[i] and nums[j] share a common factor greater than 1.
//
// Return the size of the largest connected component in the graph.
//
// Example 1:
//
// Input: nums = [4,6,15,35]
// Output: 4
//
// Example 2:
//
// Input: nums = [20,50,9,63]
// Output: 2
//
// Example 3:
//
// Input: nums = [2,3,6,7,4,12,21,39]
// Output: 8
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - 1 <= nums[i] <= 10^5
// - All the values of nums are unique.
//

struct Solution;

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        struct UnionFind {
            parent: Vec<usize>,
        }

        impl UnionFind {
            fn new(n: usize) -> Self {
                Self {
                    parent: (0..n).collect(),
                }
            }
            fn union(&mut self, x: usize, y: usize) {
                let x = self.find(x);
                let y = self.find(y);
                self.parent[y] = x
            }
            fn find(&mut self, x: usize) -> usize {
                if x != self.parent[x] {
                    self.parent[x] = self.find(self.parent[x]);
                }
                self.parent[x]
            }
        }

        let mut sieve = (0..100_001).collect::<Vec<_>>();
        for i in (2..).take_while(|&i| i * i < 100_001) {
            if sieve[i] == i as i32 {
                for j in (i..100_001).step_by(i) {
                    sieve[j] = sieve[j].min(i as i32);
                }
            }
        }
        let mut hm = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let mut n = num;
            while n > 1 {
                let p = sieve[n as usize];
                hm.entry(p).or_insert_with(Vec::new).push(i);
                n /= p;
            }
        }
        let mut uf = UnionFind::new(nums.len());
        for v in hm.values_mut() {
            v.dedup();
            v.windows(2).for_each(|w| uf.union(w[0], w[1]));
        }
        let mut counts = vec![0; nums.len()];
        for i in 0..nums.len() {
            counts[uf.find(i)] += 1;
        }
        *counts.iter().max().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_component_size(vec![4, 6, 15, 35]), 4);
    assert_eq!(Solution::largest_component_size(vec![20, 50, 9, 63]), 2);
    assert_eq!(Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]), 8);
}
