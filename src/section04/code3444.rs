#![allow(dead_code)]

// 3444. Minimum Increments for Target Multiples in an Array
// https://leetcode.com/problems/minimum-increments-for-target-multiples-in-an-array/
// https://leetcode.cn/problems/minimum-increments-for-target-multiples-in-an-array/
//
// Hard
//
// You are given two arrays, nums and target.
//
// In a single operation, you may increment any element of nums by 1.
//
// Return the minimum number of operations required so that each element in target has at least one multiple in nums.
//
// Example 1:
//
// Input: nums = [1,2,3], target = [4]
//
// Output: 1
//
// Explanation:
//
// The minimum number of operations required to satisfy the condition is 1.
//
//     Increment 3 to 4 with just one operation, making 4 a multiple of itself.
//
// Example 2:
//
// Input: nums = [8,4], target = [10,5]
//
// Output: 2
//
// Explanation:
//
// The minimum number of operations required to satisfy the condition is 2.
//
//     Increment 8 to 10 with 2 operations, making 10 a multiple of both 5 and 10.
//
// Example 3:
//
// Input: nums = [7,9,10], target = [7]
//
// Output: 0
//
// Explanation:
//
// Target 7 already has a multiple in nums, so no additional operations are needed.
//
// Constraints:
//
//     1 <= nums.length <= 5 * 10^4
//     1 <= target.length <= 4
//     target.length <= nums.length
//     1 <= nums[i], target[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        fn gcd<T: Eq + Default + Copy + std::ops::Rem<Output = T>>(a: T, b: T) -> T {
            if b == T::default() { a } else { gcd(b, a % b) }
        }
        fn lcm(a: i64, b: i64) -> i64 {
            a.checked_div(gcd(a, b)).and_then(|x| x.checked_mul(b)).unwrap_or(i64::MAX)
        }
        fn solve(nums: &Vec<i64>, _target: &Vec<i64>, mask: i64, id: i64, dp: &mut Vec<Vec<i64>>, mask_to_lcm: &Vec<i64>, n: i64) -> i64 {
            if mask == 0 {
                return 0;
            }
            if id == n {
                return i32::MAX as i64;
            }
            if dp[mask as usize][id as usize] != -1 {
                return dp[mask as usize][id as usize];
            }
            let mut res = solve(nums, _target, mask, id + 1, dp, mask_to_lcm, n);
            let mut sub = mask;
            while sub > 0 {
                let lc = mask_to_lcm[sub as usize];
                let mut x = nums[id as usize] % lc;
                if x > 0 {
                    x = lc - x;
                }
                res = res.min(x + solve(nums, _target, mask ^ sub, id + 1, dp, mask_to_lcm, n));
                sub = (sub - 1) & mask;
            }
            dp[mask as usize][id as usize] = res;
            res
        }

        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let target: Vec<i64> = target.iter().map(|&x| x as i64).collect();

        let n = nums.len() as i64;
        let m = target.len() as i64;
        let mm = 1 << m;
        let mut dp = vec![vec![-1; n as usize]; mm as usize];
        let mut mask_to_lcm = vec![0; mm as usize];
        for mask in 1..mm {
            mask_to_lcm[mask as usize] = 1;
            for i in 0..m {
                if (1 << i) & mask > 0 {
                    mask_to_lcm[mask as usize] = lcm(mask_to_lcm[mask as usize], target[i as usize]);
                }
            }
        }
        solve(&nums, &target, mm - 1, 0, &mut dp, &mask_to_lcm, n) as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let target = vec![4];
    let res = 1;
    assert_eq!(Solution::minimum_increments(nums, target), res);

    let nums = vec![8, 4];
    let target = vec![10, 5];
    let res = 2;
    assert_eq!(Solution::minimum_increments(nums, target), res);

    let nums = vec![7, 9, 10];
    let target = vec![7];
    let res = 0;
    assert_eq!(Solution::minimum_increments(nums, target), res);
}
