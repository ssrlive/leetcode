#![allow(dead_code)]

// 1000. Minimum Cost to Merge Stones
// https://leetcode.com/problems/minimum-cost-to-merge-stones/
// https://leetcode.cn/problems/minimum-cost-to-merge-stones/
//
// There are n piles of stones arranged in a row. The ith pile has stones[i] stones.
//
// A move consists of merging exactly k consecutive piles into one pile, and the cost of this move is equal to the total number of stones in these k piles.
//
// Return the minimum cost to merge all piles of stones into one pile. If it is impossible, return -1.
//
// Example 1:
//
// Input: stones = [3,2,4,1], k = 2
// Output: 20
// Explanation: We start with [3, 2, 4, 1].
// We merge [3, 2] for a cost of 5, and we are left with [5, 4, 1].
// We merge [4, 1] for a cost of 5, and we are left with [5, 5].
// We merge [5, 5] for a cost of 10, and we are left with [10].
// The total cost was 20, and this is the minimum possible.
//
// Example 2:
//
// Input: stones = [3,2,4,1], k = 3
// Output: -1
// Explanation: After any merge operation, there are 2 piles left, and we can't merge anymore.  So the task is impossible.
//
// Example 3:
//
// Input: stones = [3,5,1,2,6], k = 3
// Output: 25
// Explanation: We start with [3, 5, 1, 2, 6].
// We merge [5, 1, 2] for a cost of 8, and we are left with [3, 8, 6].
// We merge [3, 8, 6] for a cost of 17, and we are left with [17].
// The total cost was 25, and this is the minimum possible.
//
// Constraints:
//
// - n == stones.length
// - 1 <= n <= 30
// - 1 <= stones[i] <= 100
// - 2 <= k <= 30
//

struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (stones.len(), k as usize);
        if (n - 1) % (k - 1) != 0 {
            return -1;
        }

        let mut sum = vec![0; n + 1];
        let mut dp = vec![vec![vec![-1; k + 1]; n]; n];

        for i in 0..n {
            sum[i + 1] = stones[i] + sum[i];
            dp[i][i][1] = 0;
        }
        Self::eval(&mut dp, 0, n - 1, k, &sum, 1)
    }

    fn eval(dp: &mut Vec<Vec<Vec<i32>>>, i: usize, j: usize, k: usize, sum: &Vec<i32>, sz: usize) -> i32 {
        if dp[i][j][sz] != -1 {
            return dp[i][j][sz];
        }

        dp[i][j][sz] = i32::MAX;
        if sz == 1 {
            for it in i..j {
                let v1 = Self::eval(dp, i, it, k, sum, 1);
                let v2 = Self::eval(dp, it + 1, j, k, sum, k - 1);
                if v1 == i32::MAX || v2 == i32::MAX {
                    continue;
                }

                dp[i][j][sz] = dp[i][j][sz].min(v1 + v2);
            }

            if dp[i][j][sz] != i32::MAX {
                dp[i][j][sz] += sum[j + 1] - sum[i];
            }

            return dp[i][j][sz];
        }

        for it in i..j {
            let v1 = Self::eval(dp, i, it, k, sum, 1);
            let v2 = Self::eval(dp, it + 1, j, k, sum, sz - 1);
            if v1 == i32::MAX || v2 == i32::MAX {
                continue;
            }

            dp[i][j][sz] = dp[i][j][sz].min(v1 + v2);
        }

        dp[i][j][sz]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 2), 20);
    assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 3), -1);
    assert_eq!(Solution::merge_stones(vec![3, 5, 1, 2, 6], 3), 25);
}
