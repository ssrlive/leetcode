#![allow(dead_code)]

// 3334. Find the Maximum Factor Score of Array
// https://leetcode.com/problems/find-the-maximum-factor-score-of-array/
// https://leetcode.cn/problems/find-the-maximum-factor-score-of-array/
//
// Medium
//
// You are given an integer array nums.
//
// The factor score of an array is defined as the product of the LCM and GCD of all elements of that array.
//
// Return the maximum factor score of nums after removing at most one element from it.
//
// Note that both the LCM and GCD of a single number are the number itself, and the factor score of an empty array is 0.
//
// Example 1:
//
// Input: nums = [2,4,8,16]
//
// Output: 64
//
// Explanation:
//
// On removing 2, the GCD of the rest of the elements is 4 while the LCM is 16, which gives a maximum factor score of 4 * 16 = 64.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5]
//
// Output: 60
//
// Explanation:
//
// The maximum factor score of 60 can be obtained without removing any elements.
//
// Example 3:
//
// Input: nums = [3]
//
// Output: 9
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 30
//

struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n == 1 {
            return (nums[0] as i64) * (nums[0] as i64);
        }

        let mut maxi = Solution::get(&nums, -1);

        for i in 0..n {
            maxi = maxi.max(Solution::get(&nums, i as i32));
        }

        maxi
    }

    fn get(nums: &[i32], x: i32) -> i64 {
        let n = nums.len();
        let mut gcd = if x != 0 { nums[0] } else { nums[1] } as i64;
        let mut lcm = gcd;

        for (i, &nums_i) in nums.iter().enumerate().take(n).skip(if x == 0 { 2 } else { 1 }) {
            if i as i32 == x {
                continue;
            }
            let a = nums_i as i64;
            gcd = Self::__gcd(gcd, a);
            lcm = (lcm * a) / Self::__gcd(lcm, a);
        }

        gcd * lcm
    }

    fn __gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Solution::__gcd(b, a % b)
        }
    }
}

#[test]
fn test() {
    let nums = vec![2, 4, 8, 16];
    let output = 64;
    assert_eq!(Solution::max_score(nums), output);

    let nums = vec![1, 2, 3, 4, 5];
    let output = 60;
    assert_eq!(Solution::max_score(nums), output);

    let nums = vec![3];
    let output = 9;
    assert_eq!(Solution::max_score(nums), output);
}
