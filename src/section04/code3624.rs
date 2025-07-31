#![allow(dead_code)]

// 3624. Number of Integers With Popcount-Depth Equal to K II
// https://leetcode.com/problems/number-of-integers-with-popcount-depth-equal-to-k-ii/
// https://leetcode.cn/problems/number-of-integers-with-popcount-depth-equal-to-k-ii/
//
// Hard
//
// You are given an integer array nums.
//
// For any positive integer x, define the following sequence:
//
// p0 = x
// pi+1 = popcount(pi) for all i >= 0, where popcount(y) is the number of set bits (1's) in the binary representation of y.
// This sequence will eventually reach the value 1.
//
// The popcount-depth of x is defined as the smallest integer d >= 0 such that pd = 1.
//
// For example, if x = 7 (binary representation "111"). Then, the sequence is: 7 → 3 → 2 → 1, so the popcount-depth of 7 is 3.
//
// You are also given a 2D integer array queries, where each queries[i] is either:
//
// [1, l, r, k] - Determine the number of indices j such that l <= j <= r and the popcount-depth of nums[j] is equal to k.
// [2, idx, val] - Update nums[idx] to val.
// Return an integer array answer, where answer[i] is the number of indices for the ith query of type [1, l, r, k].
//
// Example 1:
//
// Input: nums = [2,4], queries = [[1,0,1,1],[2,1,1],[1,0,1,0]]
//
// Output: [2,1]
//
// Explanation:
//
// i	queries[i]	nums	binary(nums)	popcount-
// depth	[l, r]	k	Valid
// nums[j]	updated
// nums	Answer
// 0	[1,0,1,1]	[2,4]	[10, 100]	[1, 1]	[0, 1]	1	[0, 1]	—	2
// 1	[2,1,1]	[2,4]	[10, 100]	[1, 1]	—	—	—	[2,1]	—
// 2	[1,0,1,0]	[2,1]	[10, 1]	[1, 0]	[0, 1]	0	[1]	—	1
// Thus, the final answer is [2, 1].
//
// Example 2:
//
// Input: nums = [3,5,6], queries = [[1,0,2,2],[2,1,4],[1,1,2,1],[1,0,1,0]]
//
// Output: [3,1,0]
//
// Explanation:
//
// i	queries[i]	nums	binary(nums)	popcount-
// depth	[l, r]	k	Valid
// nums[j]	updated
// nums	Answer
// 0	[1,0,2,2]	[3, 5, 6]	[11, 101, 110]	[2, 2, 2]	[0, 2]	2	[0, 1, 2]	—	3
// 1	[2,1,4]	[3, 5, 6]	[11, 101, 110]	[2, 2, 2]	—	—	—	[3, 4, 6]	—
// 2	[1,1,2,1]	[3, 4, 6]	[11, 100, 110]	[2, 1, 2]	[1, 2]	1	[1]	—	1
// 3	[1,0,1,0]	[3, 4, 6]	[11, 100, 110]	[2, 1, 2]	[0, 1]	0	[]	—	0
// Thus, the final answer is [3, 1, 0].
//
// Example 3:
//
// Input: nums = [1,2], queries = [[1,0,1,1],[2,0,3],[1,0,0,1],[1,0,0,2]]
//
// Output: [1,0,1]
//
// Explanation:
//
// i	queries[i]	nums	binary(nums)	popcount-
// depth	[l, r]	k	Valid
// nums[j]	updated
// nums	Answer
// 0	[1,0,1,1]	[1, 2]	[1, 10]	[0, 1]	[0, 1]	1	[1]	—	1
// 1	[2,0,3]	[1, 2]	[1, 10]	[0, 1]	—	—	—	[3, 2]
// 2	[1,0,0,1]	[3, 2]	[11, 10]	[2, 1]	[0, 0]	1	[]	—	0
// 3	[1,0,0,2]	[3, 2]	[11, 10]	[2, 1]	[0, 0]	2	[0]	—	1
// Thus, the final answer is [1, 0, 1].
//
// Constraints:
//
// 1 <= n == nums.length <= 10^5
// 1 <= nums[i] <= 10^15
// 1 <= queries.length <= 10^5
// queries[i].length == 3 or 4
// queries[i] == [1, l, r, k] or,
// queries[i] == [2, idx, val]
// 0 <= l <= r <= n - 1
// 0 <= k <= 5
// 0 <= idx <= n - 1
// 1 <= val <= 10^15
//

struct Solution;

struct SegmentTree {
    n: usize,
    tree: Vec<[i32; 6]>,
}

impl SegmentTree {
    fn get_depth(mut x: i64) -> usize {
        let mut d = 0;
        while x > 1 {
            x = x.count_ones() as i64;
            d += 1;
        }
        d
    }

    fn build(&mut self, l: usize, r: usize, node: usize, nums: &Vec<i64>) {
        if l == r {
            let d = Self::get_depth(nums[l]);
            self.tree[node][d] = 1;
            return;
        }
        let m = (l + r) / 2;
        self.build(l, m, node * 2, nums);
        self.build(m + 1, r, node * 2 + 1, nums);
        for i in 0..6 {
            self.tree[node][i] = self.tree[node * 2][i] + self.tree[node * 2 + 1][i];
        }
    }

    fn update(&mut self, idx: usize, val: i64, l: usize, r: usize, node: usize) {
        if l == r {
            self.tree[node] = [0; 6];
            let d = Self::get_depth(val);
            self.tree[node][d] = 1;
            return;
        }
        let m = (l + r) / 2;
        if idx <= m {
            self.update(idx, val, l, m, node * 2);
        } else {
            self.update(idx, val, m + 1, r, node * 2 + 1);
        }
        for i in 0..6 {
            self.tree[node][i] = self.tree[node * 2][i] + self.tree[node * 2 + 1][i];
        }
    }

    fn query(&self, ql: usize, qr: usize, l: usize, r: usize, node: usize, k: usize) -> i32 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node][k];
        }
        let m = (l + r) / 2;
        self.query(ql, qr, l, m, node * 2, k) + self.query(ql, qr, m + 1, r, node * 2 + 1, k)
    }

    fn new(nums: &Vec<i64>) -> Self {
        let n = nums.len();
        let tree = vec![[0; 6]; 4 * n];
        let mut seg = SegmentTree { n, tree };
        seg.build(0, n - 1, 1, nums);
        seg
    }

    fn point_update(&mut self, idx: usize, val: i64) {
        self.update(idx, val, 0, self.n - 1, 1);
    }

    fn get_count(&self, l: usize, r: usize, k: usize) -> i32 {
        self.query(l, r, 0, self.n - 1, 1, k)
    }
}

impl Solution {
    pub fn popcount_depth(nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
        let mut tree = SegmentTree::new(&nums);
        let mut res = vec![];

        for q in queries {
            if q[0] == 1 {
                let (l, r, k) = (q[1] as usize, q[2] as usize, q[3] as usize);
                res.push(tree.get_count(l, r, k));
            } else {
                let (idx, val) = (q[1] as usize, q[2]);
                tree.point_update(idx, val);
            }
        }

        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 4];
    let queries = vec![vec![1, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]];
    assert_eq!(Solution::popcount_depth(nums, queries), vec![2, 1]);

    let nums = vec![3, 5, 6];
    let queries = vec![vec![1, 0, 2, 2], vec![2, 1, 4], vec![1, 1, 2, 1], vec![1, 0, 1, 0]];
    assert_eq!(Solution::popcount_depth(nums, queries), vec![3, 1, 0]);

    let nums = vec![1, 2];
    let queries = vec![vec![1, 0, 1, 1], vec![2, 0, 3], vec![1, 0, 0, 1], vec![1, 0, 0, 2]];
    assert_eq!(Solution::popcount_depth(nums, queries), vec![1, 0, 1]);
}
