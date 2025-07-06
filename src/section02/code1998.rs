#![allow(dead_code)]

/*

// 1998. GCD Sort of an Array
// https://leetcode.com/problems/gcd-sort-of-an-array/
// https://leetcode.cn/problems/gcd-sort-of-an-array/
//
// Hard
//
// You are given an integer array nums, and you can perform the following operation any number of times on nums:

    Swap the positions of two elements nums[i] and nums[j] if gcd(nums[i], nums[j]) > 1 where gcd(nums[i], nums[j]) is the greatest common divisor of nums[i] and nums[j].

Return true if it is possible to sort nums in non-decreasing order using the above swap method, or false otherwise.

Example 1:

Input: nums = [7,21,3]
Output: true
Explanation: We can sort [7,21,3] by performing the following operations:
- Swap 7 and 21 because gcd(7,21) = 7. nums = [21,7,3]
- Swap 21 and 3 because gcd(21,3) = 3. nums = [3,7,21]

Example 2:

Input: nums = [5,2,6,2]
Output: false
Explanation: It is impossible to sort the array because 5 cannot be swapped with any other element.

Example 3:

Input: nums = [10,5,9,3,15]
Output: true
We can sort [10,5,9,3,15] by performing the following operations:
- Swap 10 and 15 because gcd(10,15) = 5. nums = [15,5,9,3,10]
- Swap 15 and 3 because gcd(15,3) = 3. nums = [3,5,9,15,10]
- Swap 10 and 15 because gcd(10,15) = 5. nums = [3,5,9,10,15]

Constraints:

    1 <= nums.length <= 3 * 10^4
    2 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        fn prime_set(n: usize) -> HashSet<usize> {
            for i in 2..((n as f64).sqrt() as usize + 1) {
                if n.is_multiple_of(i) {
                    let mut set = prime_set(n / i);
                    set.insert(i);
                    return set;
                }
            }
            HashSet::from([n])
        }

        let len = nums.len();
        let mut uf = UnionFind::new(len);

        let mut indexes = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            indexes.insert(*v, i);
        }

        let sorted = {
            let mut sorted = nums.clone();
            sorted.sort_unstable();
            sorted
        };

        let mut primes: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let prime_set = prime_set(num as usize);
            for p in prime_set {
                (*primes.entry(p).or_default()).push(i);
            }
        }

        for (_, v) in primes {
            for i in 1..v.len() {
                uf.union(v[i - 1], v[i]);
            }
        }

        for i in 0..len {
            if uf.find(indexes[&sorted[i]]) != uf.find(i) {
                return false;
            }
        }

        true
    }
}

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        UnionFind {
            parents: (0..len).collect(),
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parents[node] == node {
            node
        } else {
            self.parents[node] = self.find(self.parents[node]);
            self.parents[node]
        }
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        self.parents[a] = b;
    }
}

#[test]
fn test() {
    assert!(Solution::gcd_sort(vec![7, 21, 3]));
    assert!(!Solution::gcd_sort(vec![5, 2, 6, 2]));
    assert!(Solution::gcd_sort(vec![10, 5, 9, 3, 15]));
    assert!(Solution::gcd_sort(vec![2, 5, 6, 10, 15]));
}
