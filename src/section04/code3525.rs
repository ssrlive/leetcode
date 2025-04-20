#![allow(dead_code)]

// 3525. Find X Value of Array II
// https://leetcode.com/problems/find-x-value-of-array-ii/
// https://leetcode.cn/problems/find-x-value-of-array-ii/
//
// Hard
//
// You are given an array of positive integers nums and a positive integer k. You are also given a 2D array queries, where queries[i] = [indexi, valuei, starti, xi].
//
// You are allowed to perform an operation once on nums, where you can remove any suffix from nums such that nums remains non-empty.
//
// The x-value of nums for a given x is defined as the number of ways to perform this operation so that the product of the remaining elements leaves a remainder of x modulo k.
//
// For each query in queries you need to determine the x-value of nums for xi after performing the following actions:
//
//     Update nums[indexi] to valuei. Only this step persists for the rest of the queries.
//     Remove the prefix nums[0..(starti - 1)] (where nums[0..(-1)] will be used to represent the empty prefix).
//
// Return an array result of size queries.length where result[i] is the answer for the ith query.
//
// A prefix of an array is a that starts from the beginning of the array and extends to any point within it.
//
// A suffix of an array is a that starts at any point within the array and extends to the end of the array.
//
// Note that the prefix and suffix to be chosen for the operation can be empty.
//
// Note that x-value has a different definition in this version.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5], k = 3, queries = [[2,2,0,2],[3,3,3,0],[0,1,0,1]]
//
// Output: [2,2,2]
//
// Explanation:
//
//     For query 0, nums becomes [1, 2, 2, 4, 5], and the empty prefix must be removed. The possible operations are:
//         Remove the suffix [2, 4, 5]. nums becomes [1, 2].
//         Remove the empty suffix. nums becomes [1, 2, 2, 4, 5] with a product 80, which gives remainder 2 when divided by 3.
//     For query 1, nums becomes [1, 2, 2, 3, 5], and the prefix [1, 2, 2] must be removed. The possible operations are:
//         Remove the empty suffix. nums becomes [3, 5].
//         Remove the suffix [5]. nums becomes [3].
//     For query 2, nums becomes [1, 2, 2, 3, 5], and the empty prefix must be removed. The possible operations are:
//         Remove the suffix [2, 2, 3, 5]. nums becomes [1].
//         Remove the suffix [3, 5]. nums becomes [1, 2, 2].
//
// Example 2:
//
// Input: nums = [1,2,4,8,16,32], k = 4, queries = [[0,2,0,2],[0,2,0,1]]
//
// Output: [1,0]
//
// Explanation:
//
//     For query 0, nums becomes [2, 2, 4, 8, 16, 32]. The only possible operation is:
//         Remove the suffix [2, 4, 8, 16, 32].
//     For query 1, nums becomes [2, 2, 4, 8, 16, 32]. There is no possible way to perform the operation.
//
// Example 3:
//
// Input: nums = [1,1,2,1,1], k = 2, queries = [[2,1,0,1]]
//
// Output: [5]
//
// Constraints:
//
//     1 <= nums[i] <= 10^9
//     1 <= nums.length <= 10^5
//     1 <= k <= 5
//     1 <= queries.length <= 2 * 10^4
//     queries[i] == [indexi, valuei, starti, xi]
//     0 <= indexi <= nums.length - 1
//     1 <= valuei <= 10^9
//     0 <= starti <= nums.length - 1
//     0 <= xi <= k - 1
//

struct Solution;

#[derive(Clone)]
struct Node {
    cnt: Vec<i32>,
    prod: i32,
}

impl Node {
    fn new(k: usize) -> Self {
        Self { cnt: vec![0; k], prod: 1 }
    }
}

struct SegTree {
    k: i32,
    n: usize,
    s: usize,
    tree: Vec<Node>,
}
impl SegTree {
    fn new(nums: &[i32], k: i32) -> Self {
        let n = nums.len();
        let mut s = 1;
        while s < n {
            s <<= 1;
        }
        let mut tree = vec![Node::new(k as usize); 2 * s];

        for i in 0..n {
            let a_mod = nums[i] % k;
            tree[s + i].cnt[a_mod as usize] = 1;
            tree[s + i].prod = a_mod;
        }

        for p in (1..s).rev() {
            tree[p] = Self::merge(&tree[2 * p], &tree[2 * p + 1]);
        }

        Self { k, n, s, tree }
    }

    fn merge(l: &Node, r: &Node) -> Node {
        let mut res = Node::new(l.cnt.len());
        res.prod = (l.prod * r.prod) % l.cnt.len() as i32;

        for (i, &c) in l.cnt.iter().enumerate() {
            res.cnt[i] += c;
        }
        for r_b in 0..l.cnt.len() {
            let c = r.cnt[r_b];
            if c > 0 {
                let r_ = (l.prod * r_b as i32) % l.cnt.len() as i32;
                res.cnt[r_ as usize] += c;
            }
        }
        res
    }

    fn update(&mut self, idx: usize, val: i32) {
        let pos = self.s + idx;
        let a_mod = val % self.k;
        self.tree[pos].cnt.fill(0);
        self.tree[pos].cnt[a_mod as usize] = 1;
        self.tree[pos].prod = a_mod;

        let mut pos = pos >> 1;
        while pos >= 1 {
            self.tree[pos] = Self::merge(&self.tree[2 * pos], &self.tree[2 * pos + 1]);
            pos >>= 1;
        }
    }

    fn query(&self, l: usize, r: usize) -> Node {
        let mut cnt_l = Node::new(self.k as usize);
        let mut cnt_r = Node::new(self.k as usize);
        cnt_l.prod = 1;
        cnt_r.prod = 1;
        let mut l = l + self.s;
        let mut r = r + self.s;
        while l < r {
            if l & 1 == 1 {
                cnt_l = Self::merge(&cnt_l, &self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                cnt_r = Self::merge(&self.tree[r], &cnt_r);
            }
            l >>= 1;
            r >>= 1;
        }
        Self::merge(&cnt_l, &cnt_r)
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut st = SegTree::new(&nums, k);
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let (idx, val, start, x) = (q[0], q[1], q[2], q[3]);
            st.update(idx as usize, val);
            let result = st.query(start as usize, nums.len());
            res.push(result.cnt[x as usize]);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let k = 3;
    let queries = vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]];
    assert_eq!(Solution::result_array(nums.clone(), k, queries), vec![2, 2, 2]);

    let nums = vec![1, 2, 4, 8, 16, 32];
    let k = 4;
    let queries = vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]];
    assert_eq!(Solution::result_array(nums.clone(), k, queries), vec![1, 0]);

    let nums = vec![1, 1, 2, 1, 1];
    let k = 2;
    let queries = vec![vec![2, 1, 0, 1]];
    assert_eq!(Solution::result_array(nums.clone(), k, queries), vec![5]);
}
