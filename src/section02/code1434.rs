#![allow(dead_code)]

// 1434. Number of Ways to Wear Different Hats to Each Other
// https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
// https://leetcode.cn/problems/number-of-ways-to-wear-different-hats-to-each-other/
//
// Hard
//
// There are n people and 40 types of hats labeled from 1 to 40.
//
// Given a 2D integer array hats, where hats[i] is a list of all hats preferred by the ith person.
//
// Return the number of ways that the n people wear different hats to each other.
//
// Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: hats = [[3,4],[4,5],[5]]
// Output: 1
// Explanation: There is only one way to choose hats given the conditions.
// First person choose hat 3, Second person choose hat 4 and last one hat 5.
//
// Example 2:
//
// Input: hats = [[3,5,1],[3,5]]
// Output: 4
// Explanation: There are 4 ways to choose hats:
// (3,5), (5,3), (1,3) and (1,5)
//
// Example 3:
//
// Input: hats = [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]
// Output: 24
// Explanation: Each person can choose hats labeled from 1 to 4.
// Number of Permutations of (1,2,3,4) = 24.
//
// Constraints:
//
// -    n == hats.length
// -    1 <= n <= 10
// -    1 <= hats[i].length <= 40
// -    1 <= hats[i][j] <= 40
// -    hats[i] contains a list of unique integers.
//

struct Solution;

impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        fn dfs(h2p: &Vec<Vec<usize>>, all_mask: i32, hat: usize, assigned_people: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
            if assigned_people == all_mask {
                return 1;
            }
            if hat > 40 {
                return 0;
            }
            if dp[hat][assigned_people as usize] != -1 {
                return dp[hat][assigned_people as usize];
            }
            let mut ans = dfs(h2p, all_mask, hat + 1, assigned_people, dp);
            for &p in &h2p[hat] {
                if ((assigned_people >> p) & 1) == 1 {
                    continue;
                }
                ans += dfs(h2p, all_mask, hat + 1, assigned_people | (1 << p), dp);
                ans %= 1_000_000_007;
            }
            dp[hat][assigned_people as usize] = ans;
            ans
        }

        let n = hats.len();
        let mut h2p = vec![vec![]; 41];
        for (i, item) in hats.iter().enumerate().take(n) {
            for &hat in item {
                h2p[hat as usize].push(i);
            }
        }
        let mut dp = vec![vec![-1; 1024]; 41];
        dfs(&h2p, (1 << n) - 1, 1, 0, &mut dp)
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![3, 4], vec![4, 5], vec![5]], 1),
        (vec![vec![3, 5, 1], vec![3, 5]], 4),
        (
            vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4]],
            24,
        ),
    ];
    for (hats, expected) in cases {
        assert_eq!(Solution::number_ways(hats), expected);
    }
}
