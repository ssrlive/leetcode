#![allow(dead_code)]

// 3411. Maximum Subarray With Equal Products
// https://leetcode.com/problems/maximum-subarray-with-equal-products/
// https://leetcode.cn/problems/maximum-subarray-with-equal-products/
//
// Easy
//
// You are given an array of positive integers nums.
//
// An array arr is called product equivalent if prod(arr) == lcm(arr) * gcd(arr), where:
//
//     prod(arr) is the product of all elements of arr.
//     gcd(arr) is the GCD of all elements of arr.
//     lcm(arr) is the LCM of all elements of arr.
//
// Return the length of the longest product equivalent subarray of nums.
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
// The term gcd(a, b) denotes the greatest common divisor of a and b.
//
// The term lcm(a, b) denotes the least common multiple of a and b.
//
// Example 1:
//
// Input: nums = [1,2,1,2,1,1,1]
//
// Output: 5
//
// Explanation:
//
// The longest product equivalent subarray is [1, 2, 1, 1, 1], where prod([1, 2, 1, 1, 1]) = 2,
// gcd([1, 2, 1, 1, 1]) = 1, and lcm([1, 2, 1, 1, 1]) = 2.
//
// Example 2:
//
// Input: nums = [2,3,4,5,6]
//
// Output: 3
//
// Explanation:
//
// The longest product equivalent subarray is [3, 4, 5].
//
// Example 3:
//
// Input: nums = [1,2,3,1,4,5,1]
//
// Output: 5
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 <= nums[i] <= 10
//

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        fn lcm(a: i64, b: i64) -> i64 {
            a / gcd(a, b) * b
        }

        let mut ans = 0;
        for i in 0..nums.len() {
            let mut gcd_val = nums[i] as i64;
            let mut lcm_val = nums[i] as i64;
            let mut prod = 1;
            let max_lcm = 100000000000;
            for (j, &nums_j) in nums.iter().enumerate().skip(i) {
                prod *= nums_j as i64;
                if prod > max_lcm {
                    break;
                }
                gcd_val = gcd(gcd_val, nums_j as i64);
                lcm_val = lcm(lcm_val, nums_j as i64);
                if prod == gcd_val * lcm_val {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1, 2, 1, 1, 1];
    let res = 5;
    assert_eq!(Solution::max_length(nums), res);

    let nums = vec![2, 3, 4, 5, 6];
    let res = 3;
    assert_eq!(Solution::max_length(nums), res);

    let nums = vec![1, 2, 3, 1, 4, 5, 1];
    let res = 5;
    assert_eq!(Solution::max_length(nums), res);
}
