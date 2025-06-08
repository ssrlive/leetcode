#![allow(dead_code)]

// 3572. Maximize Y‑Sum by Picking a Triplet of Distinct X‑Values
// https://leetcode.com/problems/maximize-y-sum-by-picking-a-triplet-of-distinct-x-values/
// https://leetcode.cn/problems/maximize-y-sum-by-picking-a-triplet-of-distinct-x-values/
//
// Medium
//
// You are given two integer arrays x and y, each of length n.
// You must choose three distinct indices i, j, and k such that:
//
//     x[i] != x[j]
//     x[j] != x[k]
//     x[k] != x[i]
//
// Your goal is to maximize the value of y[i] + y[j] + y[k] under these conditions.
// Return the maximum possible sum that can be obtained by choosing such a triplet of indices.
//
// If no such triplet exists, return -1.
//
// Example 1:
//
// Input: x = [1,2,1,3,2], y = [5,3,4,6,2]
//
// Output: 14
//
// Explanation:
//
//     Choose i = 0 (x[i] = 1, y[i] = 5), j = 1 (x[j] = 2, y[j] = 3), k = 3 (x[k] = 3, y[k] = 6).
//     All three values chosen from x are distinct. 5 + 3 + 6 = 14 is the maximum we can obtain. Hence, the output is 14.
//
// Example 2:
//
// Input: x = [1,2,1,2], y = [4,5,6,7]
//
// Output: -1
//
// Explanation:
//
//     There are only two distinct values in x. Hence, the output is -1.
//
// Constraints:
//
//     n == x.length == y.length
//     3 <= n <= 10^5
//     1 <= x[i], y[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        let n = x.len();
        let mut hash: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        // Store max value for each unique key
        for i in 0..n {
            hash.entry(x[i]).and_modify(|v: &mut i32| *v = (*v).max(y[i])).or_insert(y[i]);
        }

        if hash.len() < 3 {
            return -1;
        }

        // Store all max values and sort
        let mut vals: Vec<i32> = hash.into_values().collect();
        vals.sort_unstable_by(|a, b| b.cmp(a));

        vals[0] + vals[1] + vals[2]
    }
}

#[test]
fn test() {
    let x = vec![1, 2, 1, 3, 2];
    let y = vec![5, 3, 4, 6, 2];
    assert_eq!(Solution::max_sum_distinct_triplet(x, y), 14);

    let x = vec![1, 2, 1, 2];
    let y = vec![4, 5, 6, 7];
    assert_eq!(Solution::max_sum_distinct_triplet(x, y), -1);
}
