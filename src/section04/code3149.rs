#![allow(dead_code)]

// 3149. Find the Minimum Cost Array Permutation
// https://leetcode.com/problems/find-the-minimum-cost-array-permutation/
// https://leetcode.cn/problems/find-the-minimum-cost-array-permutation/
//
// Hard
//
// You are given an array nums which is a permutation of [0, 1, 2, ..., n - 1].
// The score of any permutation of [0, 1, 2, ..., n - 1] named perm is defined as:
//
// score(perm) = |perm[0] - nums[perm[1]]| + |perm[1] - nums[perm[2]]| + ... + |perm[n - 1] - nums[perm[0]]|
//
// Return the permutation perm which has the minimum possible score. If multiple permutations exist with this score,
// return the one that is lexicographically smallest among them.
//
// Example 1:
//
// Input: nums = [1,0,2]
//
// Output: [0,1,2]
//
// Explanation:
//
// The lexicographically smallest permutation with minimum cost is [0,1,2]. The cost of this permutation is |0 - 0| + |1 - 2| + |2 - 1| = 2.
//
// Example 2:
//
// Input: nums = [0,2,1]
//
// Output: [0,2,1]
//
// Explanation:
//
// The lexicographically smallest permutation with minimum cost is [0,2,1]. The cost of this permutation is |0 - 1| + |2 - 2| + |1 - 0| = 2.
//
// Constraints:
//
// 2 <= n == nums.length <= 14
// nums is a permutation of [0, 1, 2, ..., n - 1].
//

struct Solution;

impl Solution {
    const SEED: usize = 10;
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v = vec![];

        let mut ans = 1e9 as i32;
        let msk = (1 << n) - 1;
        for i in 0..n {
            let mut dp = [[-1; 1 << Self::SEED]; 15];
            let id = i;
            ans = ans.min(Self::solve(&mut dp, id, i, msk ^ (1 << i), &nums));
        }

        for i in 0..n {
            let mut dp = [[-1; 1 << Self::SEED]; 15];
            let id = i;
            if ans == Self::solve(&mut dp, id, i, msk ^ (1 << i), &nums) {
                Self::build(&mut dp, id, &mut v, i, msk ^ (1 << i), &nums);
                break;
            }
        }

        v
    }

    fn solve(dp: &mut [[i32; 1 << Self::SEED]; 15], id: usize, i: usize, msk: usize, a: &Vec<i32>) -> i32 {
        if msk == 0 {
            return (i as i32 - a[id]).abs();
        }

        if dp[i][msk] != -1 {
            return dp[i][msk];
        }

        dp[i][msk] = 1_000_000_000;
        for j in 0..a.len() {
            if msk & (1 << j) != 0 {
                dp[i][msk] = dp[i][msk].min(Self::solve(dp, id, j, msk ^ (1 << j), a) + (i as i32 - a[j]).abs());
            }
        }

        dp[i][msk]
    }

    fn build(dp: &mut [[i32; 1 << Self::SEED]; 15], id: usize, v: &mut Vec<i32>, i: usize, msk: usize, a: &Vec<i32>) {
        v.push(i as i32);
        if msk == 0 {
            return;
        }

        let ans = dp[i][msk];

        for j in 0..a.len() {
            if msk & (1 << j) != 0 && ans == Self::solve(dp, id, j, msk ^ (1 << j), a) + (i as i32 - a[j]).abs() {
                Self::build(dp, id, v, j, msk ^ (1 << j), a);
                return;
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 2];
    let output = vec![0, 1, 2];
    assert_eq!(Solution::find_permutation(nums), output);

    let nums = vec![0, 2, 1];
    let output = vec![0, 2, 1];
    assert_eq!(Solution::find_permutation(nums), output);
}
