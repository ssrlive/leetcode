#![allow(dead_code)]

// 3082. Find the Sum of the Power of All Subsequences
// https://leetcode.com/problems/find-the-sum-of-the-power-of-all-subsequences/
// https://leetcode.cn/problems/find-the-sum-of-the-power-of-all-subsequences/
//
// Hard
//
// You are given an integer array nums of length n and a positive integer k.
//
// The power of an array of integers is defined as the number of subsequences with their sum equal to k.
//
// Return the sum of power of all subsequences of nums.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [1,2,3], k = 3
//
// Output: 6
//
// Explanation:
//
// There are 5 subsequences of nums with non-zero power:
//
//     The subsequence [1,2,3] has 2 subsequences with sum == 3: [1,2,3] and [1,2,3].
//     The subsequence [1,2,3] has 1 subsequence with sum == 3: [1,2,3].
//     The subsequence [1,2,3] has 1 subsequence with sum == 3: [1,2,3].
//     The subsequence [1,2,3] has 1 subsequence with sum == 3: [1,2,3].
//     The subsequence [1,2,3] has 1 subsequence with sum == 3: [1,2,3].
//
// Hence the answer is 2 + 1 + 1 + 1 + 1 = 6.
//
// Example 2:
//
// Input: nums = [2,3,3], k = 5
//
// Output: 4
//
// Explanation:
//
// There are 3 subsequences of nums with non-zero power:
//
//     The subsequence [2,3,3] has 2 subsequences with sum == 5: [2,3,3] and [2,3,3].
//     The subsequence [2,3,3] has 1 subsequence with sum == 5: [2,3,3].
//     The subsequence [2,3,3] has 1 subsequence with sum == 5: [2,3,3].
//
// Hence the answer is 2 + 1 + 1 = 4.
//
// Example 3:
//
// Input: nums = [1,2,3], k = 7
//
// Output: 0
//
// Explanation: There exists no subsequence with sum 7. Hence all subsequences of nums have power = 0.
//
// Constraints:
//
//     1 <= n <= 10^0
//     1 <= nums[i] <= 10^4
//     1 <= k <= 100
//

struct Solution;

impl Solution {
    pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let mut dp = vec![vec![vec![-1; 101]; 101]; 101];
        (Self::fun(&mut dp, n, 0, 0, 0, &nums, k as _) % 1_000_000_007) as _
    }

    fn power(x: i64, y: usize, p: i64) -> i64 {
        let mut res = 1;
        let mut x = x % p;
        if x == 0 {
            return 0;
        }
        let mut y = y;
        while y > 0 {
            if y & 1 == 1 {
                res = (res * x) % p;
            }
            y >>= 1;
            x = (x * x) % p;
        }
        res
    }

    fn fun(dp: &mut Vec<Vec<Vec<i64>>>, n: usize, ind: usize, sum: i64, cnt: i64, nums: &Vec<i64>, k: i64) -> i64 {
        if ind >= n {
            if sum == k {
                return Self::power(2, n - cnt as usize, 1_000_000_007) % 1_000_000_007;
            }
            return 0;
        }

        if dp[ind][sum as usize][cnt as usize] != -1 {
            return dp[ind][sum as usize][cnt as usize];
        }

        let sum1 = Self::fun(dp, n, ind + 1, sum, cnt, nums, k) % 1_000_000_007;
        let sum2 = if sum + nums[ind] <= k {
            Self::fun(dp, n, ind + 1, sum + nums[ind], cnt + 1, nums, k) % 1_000_000_007
        } else {
            0
        };

        dp[ind][sum as usize][cnt as usize] = (sum1 + sum2) % 1_000_000_007;
        dp[ind][sum as usize][cnt as usize]
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let k = 3;
    let res = 6;
    assert_eq!(Solution::sum_of_power(nums, k), res);

    let nums = vec![2, 3, 3];
    let k = 5;
    let res = 4;
    assert_eq!(Solution::sum_of_power(nums, k), res);

    let nums = vec![1, 2, 3];
    let k = 7;
    let res = 0;
    assert_eq!(Solution::sum_of_power(nums, k), res);
}
