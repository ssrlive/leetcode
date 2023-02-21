#![allow(dead_code)]

/*

// 1799. Maximize Score After N Operations
Hard
515
48
Companies

You are given nums, an array of positive integers of size 2 * n. You must perform n operations on this array.

In the ith operation (1-indexed), you will:

    Choose two elements, x and y.
    Receive a score of i * gcd(x, y).
    Remove x and y from nums.

Return the maximum score you can receive after performing n operations.

The function gcd(x, y) is the greatest common divisor of x and y.

Example 1:

Input: nums = [1,2]
Output: 1
Explanation: The optimal choice of operations is:
(1 * gcd(1, 2)) = 1

Example 2:

Input: nums = [3,4,6,8]
Output: 11
Explanation: The optimal choice of operations is:
(1 * gcd(3, 6)) + (2 * gcd(4, 8)) = 3 + 8 = 11

Example 3:

Input: nums = [1,2,3,4,5,6]
Output: 14
Explanation: The optimal choice of operations is:
(1 * gcd(1, 5)) + (2 * gcd(2, 4)) + (3 * gcd(3, 6)) = 1 + 4 + 9 = 14

Constraints:

    1 <= n <= 7
    nums.length == 2 * n
    1 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut dp = vec![-1; 1 << nums.len()];
        Self::eval(&nums, 0, &mut dp)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if a < b {
            return Self::gcd(b, a);
        }
        if a % b == 0 {
            return b;
        }
        Self::gcd(b, a % b)
    }

    fn eval(nums: &Vec<i32>, mask: usize, dp: &mut Vec<i32>) -> i32 {
        if dp[mask] != -1 {
            return dp[mask];
        }

        let mut k = nums.len();
        for i in 0..nums.len() {
            if mask & 1 << i != 0 {
                k -= 1;
            }
        }
        k >>= 1;
        if k == 0 {
            return 0;
        }

        dp[mask] = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if mask & 1 << i != 0 || mask & 1 << j != 0 {
                    continue;
                }
                let mask_new = mask | 1 << i | 1 << j;
                dp[mask] = dp[mask].max(k as i32 * Self::gcd(nums[i], nums[j]) + Self::eval(nums, mask_new, dp));
            }
        }

        dp[mask]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_score(vec![1, 2]), 1);
    assert_eq!(Solution::max_score(vec![3, 4, 6, 8]), 11);
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6]), 14);
}
