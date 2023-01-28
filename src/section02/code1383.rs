#![allow(dead_code)]

// 1383. Maximum Performance of a Team
// https://leetcode.com/problems/maximum-performance-of-a-team/
// https://leetcode.cn/problems/maximum-performance-of-a-team/
//
// Hard
//
// You are given two integers n and k and two integer arrays speed and efficiency both of length n.
// There are n engineers numbered from 1 to n. speed[i] and efficiency[i] represent
// the speed and efficiency of the ith engineer respectively.
//
// Choose at most k different engineers out of the n engineers to form a team with the maximum performance.
//
// The performance of a team is the sum of their engineers' speeds multiplied by the minimum efficiency among their engineers.
//
// Return the maximum performance of this team. Since the answer can be a huge number, return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
// Output: 60
// Explanation:
// We have the maximum performance of the team by selecting engineer 2 (with speed=10 and efficiency=4)
// and engineer 5 (with speed=5 and efficiency=7). That is, performance = (10 + 5) * min(4, 7) = 60.
//
// Example 2:
//
// Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 3
// Output: 68
// Explanation:
// This is the same example as the first but k = 3. We can select engineer 1, engineer 2 and engineer 5
// to get the maximum performance of the team. That is, performance = (2 + 10 + 5) * min(5, 4, 7) = 68.
//
// Example 3:
//
// Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 4
// Output: 72
//
// Constraints:
//
// -    1 <= k <= n <= 10^5
// -    speed.length == n
// -    efficiency.length == n
// -    1 <= speed[i] <= 10^5
// -    1 <= efficiency[i] <= 10^8
//

struct Solution;

impl Solution {
    pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        const DIV: i64 = 1_000_000_007;

        let mut v = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (Reverse(e as i64), s as i64))
            .collect::<Vec<_>>();
        v.sort_unstable();
        let mut bh = BinaryHeap::with_capacity(k as usize);
        let mut sum = 0;
        let mut answer = 0;
        for &(Reverse(e), s) in &v {
            sum += s;
            bh.push(Reverse(s));
            if bh.len() > k as usize {
                if let Some(Reverse(min)) = bh.pop() {
                    sum -= min;
                }
            }
            answer = answer.max(sum * e);
        }
        (answer % DIV) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2, 60),
        (6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3, 68),
        (6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4, 72),
    ];
    for (n, speed, efficiency, k, expected) in cases {
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), expected);
    }
}
