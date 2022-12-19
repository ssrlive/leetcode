#![allow(dead_code)]

// 526. Beautiful Arrangement
// https://leetcode.com/problems/beautiful-arrangement/
// https://leetcode.cn/problems/beautiful-arrangement/
//
// Suppose you have n integers labeled 1 through n. A permutation of those n integers perm (1-indexed)
// is considered a beautiful arrangement if for every i (1 <= i <= n), either of the following is true:
//
// - perm[i] is divisible by i.
// - i is divisible by perm[i].
//
// Given an integer n, return the number of the beautiful arrangements that you can construct.
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation:
// The first beautiful arrangement is [1,2]:
//     - perm[1] = 1 is divisible by i = 1
//     - perm[2] = 2 is divisible by i = 2
// The second beautiful arrangement is [2,1]:
//     - perm[1] = 2 is divisible by i = 1
//     - i = 2 is divisible by perm[2] = 1
//
// Example 2:
//
// Input: n = 1
// Output: 1
//
// Constraints:
//
// - 1 <= n <= 15
//

struct Solution;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut answer = 0;
        let mut used = vec![false; n as usize + 1];
        fn dfs(n: i32, i: i32, used: &mut Vec<bool>, answer: &mut i32) {
            if i > n {
                *answer += 1;
                return;
            }
            for j in 1..=n {
                if !used[j as usize] && (j % i == 0 || i % j == 0) {
                    used[j as usize] = true;
                    dfs(n, i + 1, used, answer);
                    used[j as usize] = false;
                }
            }
        }
        dfs(n, 1, &mut used, &mut answer);
        answer
    }
}

#[test]
fn test_count_arrangement() {
    assert_eq!(Solution::count_arrangement(2), 2);
    assert_eq!(Solution::count_arrangement(1), 1);
}
