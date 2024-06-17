#![allow(dead_code)]

// 857. Minimum Cost to Hire K Workers
// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
// https://leetcode.cn/problems/minimum-cost-to-hire-k-workers/
//
// There are n workers. You are given two integer arrays quality and wage where quality[i] is the quality
// of the ith worker and wage[i] is the minimum wage expectation for the ith worker.
//
// We want to hire exactly k workers to form a paid group. To hire a group of k workers, we must pay them according to the following rules:
//
// Every worker in the paid group should be paid in the ratio of their quality compared to other workers in the paid group.
// Every worker in the paid group must be paid at least their minimum wage expectation.
// Given the integer k, return the least amount of money needed to form a paid group satisfying the above conditions.
// Answers within 10-5 of the actual answer will be accepted.
//
// Example 1:
//
// Input: quality = [10,20,5], wage = [70,50,30], k = 2
// Output: 105.00000
// Explanation: We pay 70 to 0th worker and 35 to 2nd worker.
//
// Example 2:
//
// Input: quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
// Output: 30.66667
// Explanation: We pay 4 to 0th worker, 13.33333 to 2nd and 3rd workers separately.
//
// Constraints:
//
// - n == quality.length == wage.length
// - 1 <= k <= n <= 10^4
// - 1 <= quality[i], wage[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            let r = wage[i] as f64 / quality[i] as f64;
            v.push((r, quality[i] as f64));
        }
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut pq = std::collections::BinaryHeap::new();
        let mut qs = 0.0;
        let mut ans = f64::MAX;
        for item in v.iter() {
            qs += item.1;
            pq.push(item.1 as i64);
            if pq.len() > k as usize {
                qs -= pq.pop().unwrap() as f64;
            }
            if pq.len() == k as usize {
                ans = ans.min(qs * item.0);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let quality = vec![10, 20, 5];
    let wage = vec![70, 50, 30];
    let k = 2;
    let ans = 105.00000;
    assert_eq!(Solution::mincost_to_hire_workers(quality, wage, k), ans);

    let quality = vec![3, 1, 10, 10, 1];
    let wage = vec![4, 8, 2, 2, 7];
    let k = 3;
    let ans = 30.666666666666664;
    assert_eq!(Solution::mincost_to_hire_workers(quality, wage, k), ans);
}
