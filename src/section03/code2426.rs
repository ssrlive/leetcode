#![allow(dead_code)]

// 2426. Number of Pairs Satisfying Inequality
// https://leetcode.com/problems/number-of-pairs-satisfying-inequality/
// https://leetcode.cn/problems/number-of-pairs-satisfying-inequality/
//
// You are given two 0-indexed integer arrays nums1 and nums2, each of size n, and an integer diff. Find the number of pairs (i, j) such that:
//
// 0 <= i < j <= n - 1 and
// nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff.
//
// Return the number of pairs that satisfy the conditions.
//
// Example 1:
//
// Input: nums1 = [3,2,5], nums2 = [2,2,1], diff = 1
// Output: 3
// Explanation:
// There are 3 pairs that satisfy the conditions:
// 1. i = 0, j = 1: 3 - 2 <= 2 - 2 + 1. Since i < j and 1 <= 1, this pair satisfies the conditions.
// 2. i = 0, j = 2: 3 - 5 <= 2 - 1 + 1. Since i < j and -2 <= 2, this pair satisfies the conditions.
// 3. i = 1, j = 2: 2 - 5 <= 2 - 1 + 1. Since i < j and -3 <= 2, this pair satisfies the conditions.
// Therefore, we return 3.
//
// Example 2:
//
// Input: nums1 = [3,-1], nums2 = [-2,2], diff = -1
// Output: 0
// Explanation:
// Since there does not exist any pair that satisfies the conditions, we return 0.
//
// Constraints:
//
// - n == nums1.length == nums2.length
// - 2 <= n <= 10^5
// - -10^4 <= nums1[i], nums2[i] <= 10^4
// - -10^4 <= diff <= 10^4
//

struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        use std::collections::BTreeMap;

        let n = nums1.len();
        let mut mp = BTreeMap::<i32, i32>::new();

        for i in 0..n {
            *mp.entry(nums1[i] - nums2[i]).or_default() += 1;
        }

        let m = mp.len();
        let mut tree = vec![0; m * 4];
        let mut index = vec![];

        for (key, val) in mp {
            let i = index.len();
            Self::add(1, 0, m - 1, i, val as usize, &mut tree);
            index.push(key);
        }

        let mut ret = 0;
        for i in (0..n).rev() {
            let j = Self::binary_search(&index, nums1[i] - nums2[i]);
            Self::remove(1, 0, m - 1, j, &mut tree);
            let k = Self::binary_search(&index, nums1[i] - nums2[i] + diff + 1);
            ret += i as i64 - Self::get(1, 0, m - 1, k, &tree) as i64;
        }

        ret
    }

    fn binary_search(index: &[i32], val: i32) -> usize {
        let m = index.len();
        if index[m - 1] < val {
            return m;
        }

        let (mut left, mut right) = (0, m - 1);

        while left < right {
            let mid = left + (right - left) / 2;

            if index[mid] >= val {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn add(u: usize, left: usize, right: usize, i: usize, val: usize, tree: &mut Vec<usize>) {
        if i < left || i > right {
            return;
        }
        if left == right {
            tree[u] += val;
            return;
        }

        let mid = left + (right - left) / 2;

        if i <= mid {
            Self::add(2 * u, left, mid, i, val, tree);
        } else {
            Self::add(2 * u + 1, mid + 1, right, i, val, tree);
        }

        tree[u] = tree[2 * u] + tree[2 * u + 1];
    }

    fn remove(u: usize, left: usize, right: usize, i: usize, tree: &mut Vec<usize>) {
        if i < left || i > right {
            return;
        }
        if left == right {
            tree[u] -= 1;
            return;
        }

        let mid = left + (right - left) / 2;
        if i <= mid {
            Self::remove(2 * u, left, mid, i, tree);
        } else {
            Self::remove(2 * u + 1, mid + 1, right, i, tree);
        }

        tree[u] = tree[2 * u] + tree[2 * u + 1];
    }

    fn get(u: usize, left: usize, right: usize, i: usize, tree: &Vec<usize>) -> i32 {
        if i > right {
            return 0;
        }
        if i <= left {
            return tree[u] as i32;
        }

        let mid = left + (right - left) / 2;

        Self::get(2 * u, left, mid, i, tree) + Self::get(2 * u + 1, mid + 1, right, i, tree)
    }
}

#[test]
fn test() {
    let nums1 = vec![3, 2, 5];
    let nums2 = vec![2, 2, 1];
    let diff = 1;
    assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 3);

    let nums1 = vec![3, -1];
    let nums2 = vec![-2, 2];
    let diff = -1;
    assert_eq!(Solution::number_of_pairs(nums1, nums2, diff), 0);
}
