#![allow(dead_code)]

// 3569. Maximize Count of Distinct Primes After Split
// https://leetcode.com/problems/maximize-count-of-distinct-primes-after-split/
// https://leetcode.cn/problems/maximize-count-of-distinct-primes-after-split/
//
// Hard
//
// You are given an integer array nums having length n and a 2D integer array queries where queries[i] = [idx, val].
//
// For each query:
//
// - Update nums[idx] = val.
// - Choose an integer k with 1 <= k < n to split the array into the non-empty prefix nums[0..k-1] and suffix nums[k..n-1]
//   such that the sum of the counts of distinct values in each part is maximum.
//
// Note: The changes made to the array in one query persist into the next query.
//
// Return an array containing the result for each query, in the order they are given.
//
// Example 1:
//
// Input: nums = [2,1,3,1,2], queries = [[1,2],[3,3]]
//
// Output: [3,4]
//
// Explanation:
//
// - Initially nums = [2, 1, 3, 1, 2].
// - After 1st query, nums = [2, 2, 3, 1, 2]. Split nums into [2] and [2, 3, 1, 2]. [2] consists of 1 distinct prime
//   and [2, 3, 1, 2] consists of 2 distinct primes. Hence, the answer for this query is 1 + 2 = 3.
// - After 2nd query, nums = [2, 2, 3, 3, 2]. Split nums into [2, 2, 3] and [3, 2] with an answer of 2 + 2 = 4.
// - The output is [3, 4].
//
// Example 2:
//
// Input: nums = [2,1,4], queries = [[0,1]]
//
// Output: [0]
//
// Explanation:
//
//     Initially nums = [2, 1, 4].
//     After 1st query, nums = [1, 1, 4]. There are no prime numbers in nums, hence the answer for this query is 0.
//     The output is [0].
//
// Constraints:
//
//     2 <= n == nums.length <= 5 * 10^4
//     1 <= queries.length <= 5 * 10^4
//     1 <= nums[i] <= 10^5
//     0 <= queries[i][0] < nums.length
//     1 <= queries[i][1] <= 10^5
//

struct Solution;

struct SegmentTree {
    // Implementation of Segment Tree with lazy propagation
    n: usize,
    nodes: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    pub fn new(n: usize, init: Vec<i32>) -> Self {
        let mut nodes = vec![0; 4 * n];
        let lazy = vec![0; 4 * n];
        Self::build(&mut nodes, &init, 0, 0, n - 1);
        SegmentTree { n, nodes, lazy }
    }

    fn build(nodes: &mut Vec<i32>, init: &[i32], cur: usize, left: usize, right: usize) {
        if left == right {
            nodes[cur] = init[left];
            return;
        }
        let mid = (left + right) / 2;
        Self::build(nodes, init, 2 * cur + 1, left, mid);
        Self::build(nodes, init, 2 * cur + 2, mid + 1, right);
        nodes[cur] = std::cmp::max(nodes[2 * cur + 1], nodes[2 * cur + 2]);
    }

    fn push(&mut self, cur: usize, left: usize, right: usize) {
        if self.lazy[cur] != 0 {
            self.nodes[cur] += self.lazy[cur];
            if left != right {
                self.lazy[2 * cur + 1] += self.lazy[cur];
                self.lazy[2 * cur + 2] += self.lazy[cur];
            }
            self.lazy[cur] = 0;
        }
    }

    pub fn update(&mut self, ql: usize, qr: usize, val: i32) {
        Self::update_helper(self, 0, ql, qr, val, 0, self.n - 1);
    }

    fn update_helper(seg_tree: &mut SegmentTree, cur: usize, ql: usize, qr: usize, val: i32, l: usize, r: usize) {
        seg_tree.push(cur, l, r);
        if r < ql || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            seg_tree.lazy[cur] += val;
            seg_tree.push(cur, l, r);
            return;
        }
        let mid = (l + r) / 2;
        Self::update_helper(seg_tree, 2 * cur + 1, ql, qr, val, l, mid);
        Self::update_helper(seg_tree, 2 * cur + 2, ql, qr, val, mid + 1, r);
        seg_tree.nodes[cur] = std::cmp::max(seg_tree.nodes[2 * cur + 1], seg_tree.nodes[2 * cur + 2]);
    }

    pub fn query(&mut self) -> i32 {
        self.push(0, 0, self.n - 1);
        self.nodes[0]
    }
}

impl Solution {
    pub fn maximum_count(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MAX_NUM: usize = 100000;

        let mut nums = nums;

        // Step 1: Sieve of Eratosthenes to find all primes up to MAX_NUM
        let mut prime = vec![true; MAX_NUM + 1];
        prime[0] = false;
        prime[1] = false;
        for i in 2..=((MAX_NUM as f64).sqrt() as usize) {
            if prime[i] {
                for j in (i * i..=MAX_NUM).step_by(i) {
                    prime[j] = false;
                }
            }
        }

        // Step 2: Precompute initial state and construct hashMap: prime -> map index
        let n = nums.len();
        let mut prime_to_ind: std::collections::HashMap<i32, std::collections::BTreeSet<usize>> = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if prime[num as usize] {
                prime_to_ind.entry(num).or_default().insert(i);
            }
        }

        let mut delta = vec![0; n + 1];
        for ind_set in prime_to_ind.values() {
            if ind_set.len() >= 2 {
                let left = *ind_set.iter().next().unwrap() + 1;
                let right = *ind_set.iter().next_back().unwrap();
                delta[left] += 1;
                delta[right + 1] -= 1;
            }
        }

        for i in 1..=n {
            delta[i] += delta[i - 1];
        }

        // Step 3: Solve each query
        let mut seg_tree = SegmentTree::new(n, delta);
        let mut ans = Vec::with_capacity(queries.len());

        for q in queries {
            let ind = q[0] as usize;
            let new_val = q[1];
            let old_val = nums[ind];

            nums[ind] = new_val;
            // Remove the old value
            if prime[old_val as usize] {
                if prime_to_ind[&old_val].len() >= 2 {
                    let ind_set = prime_to_ind.get_mut(&old_val).unwrap();
                    let left = *ind_set.iter().next().unwrap() + 1;
                    let right = *ind_set.iter().next_back().unwrap();
                    if ind == left - 1 || ind == right {
                        seg_tree.update(left, right, -1);
                        ind_set.remove(&ind);
                        if ind_set.len() >= 2 {
                            let new_left = *ind_set.iter().next().unwrap() + 1;
                            let new_right = *ind_set.iter().next_back().unwrap();
                            seg_tree.update(new_left, new_right, 1);
                        }
                    } else {
                        ind_set.remove(&ind);
                    }
                } else {
                    prime_to_ind.remove(&old_val);
                }
            }
            // Add the new value
            if prime[new_val as usize] {
                if prime_to_ind.contains_key(&new_val) {
                    let ind_set = prime_to_ind.get_mut(&new_val).unwrap();
                    let left = *ind_set.iter().next().unwrap() + 1;
                    let right = *ind_set.iter().next_back().unwrap();
                    if ind < left - 1 || ind > right {
                        if ind_set.len() >= 2 {
                            let old_left = *ind_set.iter().next().unwrap() + 1;
                            let old_right = *ind_set.iter().next_back().unwrap();
                            seg_tree.update(old_left, old_right, -1);
                        }
                        ind_set.insert(ind);
                        let new_left = *ind_set.iter().next().unwrap() + 1;
                        let new_right = *ind_set.iter().next_back().unwrap();
                        seg_tree.update(new_left, new_right, 1);
                    } else {
                        ind_set.insert(ind);
                    }
                } else {
                    prime_to_ind.entry(new_val).or_default().insert(ind);
                }
            }
            ans.push(prime_to_ind.len() as i32 + seg_tree.query());
        }
        ans
    }
}

#[test]
fn test() {
    let nums = vec![2, 1, 3, 1, 2];
    let queries = vec![vec![1, 2], vec![3, 3]];
    let result = Solution::maximum_count(nums, queries);
    assert_eq!(result, vec![3, 4]);

    let nums = vec![2, 1, 4];
    let queries = vec![vec![0, 1]];
    let result = Solution::maximum_count(nums, queries);
    assert_eq!(result, vec![0]);
}
