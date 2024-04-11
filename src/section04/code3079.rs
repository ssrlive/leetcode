#![allow(dead_code)]

// 3079. Find the Sum of Encrypted Integers
// https://leetcode.com/problems/find-the-sum-of-encrypted-integers/
// https://leetcode.cn/problems/find-the-sum-of-encrypted-integers/
//
// Easy
//
// You are given an integer array nums containing positive integers. We define a function encrypt
// such that encrypt(x) replaces every digit in x with the largest digit in x.
// For example, encrypt(523) = 555 and encrypt(213) = 333.
//
// Return the sum of encrypted elements.
//
// Example 1:
//
// Input: nums = [1,2,3]
//
// Output: 6
//
// Explanation: The encrypted elements are [1,2,3]. The sum of encrypted elements is 1 + 2 + 3 == 6.
//
// Example 2:
//
// Input: nums = [10,21,31]
//
// Output: 66
//
// Explanation: The encrypted elements are [11,22,33]. The sum of encrypted elements is 11 + 22 + 33 == 66.
//
// Constraints:
//
//     1 <= nums.length <= 50
//     1 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for mut v in nums {
            let (mut cnt, mut max) = (0, 0);
            while v != 0 {
                cnt += 1;
                max = max.max(v % 10);
                v /= 10;
            }

            for _ in 0..cnt {
                res += max;
                max *= 10;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
    assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
}
