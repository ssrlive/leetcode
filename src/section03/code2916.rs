#![allow(dead_code)]

// 2916. Subarrays Distinct Element Sum of Squares II
// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii/
// https://leetcode.cn/problems/subarrays-distinct-element-sum-of-squares-ii/
//
// Hard
//
// You are given a 0-indexed integer array nums.
//
// The distinct count of a subarray of nums is defined as:
//
//     - Let nums[i..j] be a subarray of nums consisting of all the indices from i to j such that 0 <= i <= j < nums.length.
//       Then the number of distinct values in nums[i..j] is called the distinct count of nums[i..j].
//
// Return the sum of the squares of distinct counts of all subarrays of nums.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// Example 1:
//
// Input: nums = [1,2,1]
// Output: 15
// Explanation: Six possible subarrays are:
// [1]: 1 distinct value
// [2]: 1 distinct value
// [1]: 1 distinct value
// [1,2]: 2 distinct values
// [2,1]: 2 distinct values
// [1,2,1]: 2 distinct values
// The sum of the squares of the distinct counts in all subarrays is equal to 1^2 + 1^2 + 1^2 + 2^2 + 2^2 + 2^2 = 15.
//
// Example 2:
//
// Input: nums = [2,2]
// Output: 3
// Explanation: Three possible subarrays are:
// [2]: 1 distinct value
// [2]: 1 distinct value
// [2,2]: 1 distinct value
// The sum of the squares of the distinct counts in all subarrays is equal to 1^2 + 1^2 + 1^2 = 3.
//
// Constraints:
//
//     1 <= nums.length <= 10^5
//     1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut last_pos = vec![-1; 100001];

        let mut tree = SegmentTree::new(n);
        let mut ans = 0;

        for j in 0..n {
            let st = last_pos[nums[j as usize] as usize] + 1;
            let ed = j;
            tree.add_one(st, ed, 0, -1, 0);

            ans = (ans + tree.sqr[0]) % MOD;

            last_pos[nums[j as usize] as usize] = j;
        }
        ans as i32
    }
}

const MOD: i64 = 1e9 as i64 + 7;

struct SegmentTree {
    lzy: Vec<i64>,
    sum: Vec<i64>,
    sqr: Vec<i64>,
    n: i64,
}

impl SegmentTree {
    fn new(n: i64) -> Self {
        Self {
            lzy: vec![0; 4 * n as usize],
            sum: vec![0; 4 * n as usize],
            sqr: vec![0; 4 * n as usize],
            n,
        }
    }

    fn update_lzy(&mut self, l: i64, r: i64, i: i64) {
        let i = i as usize;
        if l != r {
            self.lzy[i * 2 + 1] += self.lzy[i];
            self.lzy[i * 2 + 2] += self.lzy[i];
        }
        let gap = r - l + 1;
        let new_sum = self.sum[i] + self.lzy[i] * gap;
        let new_sqr = self.sqr[i] + self.lzy[i] * self.sum[i] * 2 + self.lzy[i] * self.lzy[i] * gap;

        self.sum[i] = new_sum % MOD;
        self.sqr[i] = new_sqr % MOD;
        self.lzy[i] = 0;
    }

    fn add_one(&mut self, x: i64, y: i64, l: i64, mut r: i64, i: i64) {
        if r == -1 {
            r += self.n;
        }
        self.update_lzy(l, r, i);

        if r < x || l > y {
            return;
        }
        if l >= x && r <= y {
            self.lzy[i as usize] = 1;
            return self.update_lzy(l, r, i);
        }

        let m = (l + r) >> 1;
        self.add_one(x, y, l, m, i * 2 + 1);
        self.add_one(x, y, m + 1, r, i * 2 + 2);

        let i = i as usize;
        self.sum[i] = (self.sum[i * 2 + 1] + self.sum[i * 2 + 2]) % MOD;
        self.sqr[i] = (self.sqr[i * 2 + 1] + self.sqr[i * 2 + 2]) % MOD;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
    assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
}
