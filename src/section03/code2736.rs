#![allow(dead_code)]

// 2736. Maximum Sum Queries
// https://leetcode.com/problems/maximum-sum-queries/
// https://leetcode.cn/problems/maximum-sum-queries/
//
// Hard
//
// You are given two 0-indexed integer arrays nums1 and nums2, each of length n,
// and a 1-indexed 2D array queries where queries[i] = [xi, yi].
//
// For the ith query, find the maximum value of nums1[j] + nums2[j] among all indices j (0 <= j < n),
// where nums1[j] >= xi and nums2[j] >= yi, or -1 if there is no j satisfying the constraints.
//
// Return an array answer where answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: nums1 = [4,3,1,2], nums2 = [2,4,9,5], queries = [[4,1],[1,3],[2,5]]
// Output: [6,10,7]
// Explanation:
// For the 1st query xi = 4 and yi = 1, we can select index j = 0 since nums1[j] >= 4 and nums2[j] >= 1.
// The sum nums1[j] + nums2[j] is 6, and we can show that 6 is the maximum we can obtain.
//
// For the 2nd query xi = 1 and yi = 3, we can select index j = 2 since nums1[j] >= 1 and nums2[j] >= 3.
// The sum nums1[j] + nums2[j] is 10, and we can show that 10 is the maximum we can obtain.
//
// For the 3rd query xi = 2 and yi = 5, we can select index j = 3 since nums1[j] >= 2 and nums2[j] >= 5.
// The sum nums1[j] + nums2[j] is 7, and we can show that 7 is the maximum we can obtain.
//
// Therefore, we return [6,10,7].
//
// Example 2:
//
// Input: nums1 = [3,2,5], nums2 = [2,3,4], queries = [[4,4],[3,2],[1,1]]
// Output: [9,9,9]
// Explanation: For this example, we can use index j = 2 for all the queries since it satisfies the constraints for each query.
//
// Example 3:
//
// Input: nums1 = [2,1], nums2 = [2,3], queries = [[3,3]]
// Output: [-1]
// Explanation: There is one query in this example with xi = 3 and yi = 3.
// For every index, j, either nums1[j] < xi or nums2[j] < yi. Hence, there is no solution.
//
// Constraints:
//
//     nums1.length == nums2.length
//     n == nums1.length
//     1 <= n <= 10^5
//     1 <= nums1[i], nums2[i] <= 10^9
//     1 <= queries.length <= 10^5
//     queries[i].length == 2
//     xi == queries[i][1]
//     yi == queries[i][2]
//     1 <= xi, yi <= 10^9
//

struct Solution;

impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut s = std::collections::BTreeSet::new();
        for a in &nums2 {
            s.insert(*a);
        }

        let data = s.into_iter().collect::<Vec<i32>>();
        let (m, n, sz) = (nums1.len(), queries.len(), data.len());
        let mut tree = vec![(-1, -1); 4 * sz];
        let (mut nums, mut temp) = (vec![], vec![]);

        for i in 0..m {
            nums.push((nums1[i], nums2[i]));
        }
        for (i, query) in queries.iter().enumerate().take(n) {
            temp.push((query[0], query[1], i));
        }
        nums.sort();
        temp.sort();

        let mut k = m;
        let mut ret = vec![-1; n];
        while let Some(q) = temp.pop() {
            while k > 0 && nums[k - 1].0 >= q.0 {
                k -= 1;
                let (a, b) = (nums[k].0, nums[k].1);
                let idx = Self::binary_search(&data, b);
                Self::update(1, 0, sz - 1, idx, a + b, &mut tree);
            }
            let j = Self::binary_search(&data, q.1);
            ret[q.2] = Self::get(1, 0, sz - 1, j, &mut tree);
        }

        ret
    }

    fn update(u: usize, l: usize, r: usize, i: usize, a: i32, tree: &mut Vec<(i32, i32)>) {
        if l > i {
            return;
        }
        if r <= i {
            tree[u].0 = tree[u].0.max(a);
            tree[u].1 = tree[u].1.max(tree[u].0);
            return;
        }

        let m = l + (r - l) / 2;
        Self::update(2 * u, l, m, i, a, tree);
        Self::update(2 * u + 1, m + 1, r, i, a, tree);
        let t = tree[2 * u].1.max(tree[2 * u + 1].1);
        tree[u].1 = tree[u].1.max(t);
    }

    fn get(u: usize, l: usize, r: usize, i: usize, tree: &mut Vec<(i32, i32)>) -> i32 {
        if i > r {
            return -1;
        }
        if l >= i {
            return tree[u].0.max(tree[u].1);
        }

        if tree[u].0 != -1 {
            tree[u].1 = tree[u].1.max(tree[u].0);
            if l < r {
                tree[2 * u].0 = tree[2 * u].0.max(tree[u].0);
                tree[2 * u + 1].0 = tree[2 * u + 1].0.max(tree[u].0);
            }
            tree[u].0 = -1;
        }

        let m = l + (r - l) / 2;
        let (ret1, ret2) = (Self::get(2 * u, l, m, i, tree), Self::get(2 * u + 1, m + 1, r, i, tree));
        let t = tree[2 * u].1.max(tree[2 * u + 1].1);
        tree[u].1 = tree[u].1.max(t);

        ret1.max(ret2)
    }

    fn binary_search(data: &[i32], a: i32) -> usize {
        let (mut l, mut r) = (0, data.len() - 1);
        if a > data[r] {
            return r + 1;
        }

        while l < r {
            let m = l + (r - l) / 2;
            if data[m] < a {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l
    }
}

#[test]
fn test() {
    let nums1 = vec![4, 3, 1, 2];
    let nums2 = vec![2, 4, 9, 5];
    let queries = vec![vec![4, 1], vec![1, 3], vec![2, 5]];
    assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), vec![6, 10, 7]);

    let nums1 = vec![3, 2, 5];
    let nums2 = vec![2, 3, 4];
    let queries = vec![vec![4, 4], vec![3, 2], vec![1, 1]];
    assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), vec![9, 9, 9]);

    let nums1 = vec![2, 1];
    let nums2 = vec![2, 3];
    let queries = vec![vec![3, 3]];
    assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), vec![-1]);
}
