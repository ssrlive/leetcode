#![allow(dead_code)]

/*

// 2305. Fair Distribution of Cookies
// https://leetcode.com/problems/fair-distribution-of-cookies/
// https://leetcode.cn/problems/fair-distribution-of-cookies/
//
// Medium
//
// You are given an integer array cookies, where cookies[i] denotes the number of cookies in the ith bag. You are also given an integer k that denotes the number of children to distribute all the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.

The unfairness of a distribution is defined as the maximum total cookies obtained by a single child in the distribution.

Return the minimum unfairness of all distributions.

Example 1:

Input: cookies = [8,15,10,20,8], k = 2
Output: 31
Explanation: One optimal distribution is [8,15,8] and [10,20]
- The 1st child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
- The 2nd child receives [10,20] which has a total of 10 + 20 = 30 cookies.
The unfairness of the distribution is max(31,30) = 31.
It can be shown that there is no distribution with an unfairness less than 31.

Example 2:

Input: cookies = [6,1,3,2,2,4,1,2], k = 3
Output: 7
Explanation: One optimal distribution is [6,1], [3,2,2], and [4,1,2]
- The 1st child receives [6,1] which has a total of 6 + 1 = 7 cookies.
- The 2nd child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
- The 3rd child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
The unfairness of the distribution is max(7,7,7) = 7.
It can be shown that there is no distribution with an unfairness less than 7.

Constraints:

    2 <= cookies.length <= 8
    1 <= cookies[i] <= 10^5
    2 <= k <= cookies.length
*/

struct Solution;

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = cookies.len();
        let mut cost = vec![0; 1 << n];
        for c in 1..1 << n {
            for j in 0..n {
                if (c >> j) & 1 == 1 {
                    cost[c] = cookies[j] + cost[c ^ (1 << j)];
                    break;
                }
            }
        }
        let mut dp = vec![vec![i32::MAX; 1 << n]; k];
        dp[0][..(1 << n)].copy_from_slice(&cost[..(1 << n)]);
        for i in 1..k {
            for c1 in 0..1 << n {
                for (c2, &cost_c2) in cost.iter().enumerate().take(1 << n) {
                    if (c1 & c2) == 0 {
                        dp[i][c1 | c2] = dp[i][c1 | c2].min(dp[i - 1][c1].max(cost_c2))
                    }
                }
            }
        }
        dp[k - 1][(1 << n) - 1]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![8, 15, 10, 20, 8], 2, 31),
        (vec![6, 1, 3, 2, 2, 4, 1, 2], 3, 7),
        (vec![1, 2, 3, 4, 5, 6, 7, 8], 4, 9),
    ];
    for (cookies, k, expect) in cases {
        assert_eq!(Solution::distribute_cookies(cookies, k), expect);
    }
}
