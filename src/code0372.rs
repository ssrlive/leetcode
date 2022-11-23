#![allow(dead_code)]

// 372. Super Pow
// https://leetcode.com/problems/super-pow/
//
// Your task is to calculate ab mod 1337 where a is a positive integer and b is
// an extremely large positive integer given in the form of an array.
//
// Example 1:
// Input: a = 2, b = [3]
// Output: 8
//
// Example 2:
// Input: a = 2, b = [1,0]
// Output: 1024
//
// Example 3:
// Input: a = 1, b = [4,3,3,8,5,2]
// Output: 1
//
// Example 4:
// Input: a = 2147483647, b = [2,0,0]
// Output: 1198
//
// Note:
// 1 <= a <= 231 - 1
// 1 <= b.length <= 2000
// 0 <= b[i] <= 9
// b doesn't contain leading zeros.
//

struct Solution;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i64 = 1337;
        let a = a as i64 % MOD;
        if a == 1 {
            return 1;
        }

        if a == MOD {
            return 0;
        }

        b.into_iter().map(|b| b as i64).fold(1, |mut prev, cur| {
            prev = (prev * prev) % MOD; // square
            let prev_4 = prev * prev % MOD; // power 4
            prev = (prev * (prev_4 * prev_4)) % MOD; // power 10

            (prev
                * format!("{cur:b}")
                    .chars()
                    .rev()
                    .map(|ch| ch as u8 - b'0')
                    .fold((1, a), |(res, powa), bit| {
                        ((res * if bit == 0 { 1 } else { powa }) % MOD, (powa * powa) % MOD)
                    })
                    .0)
                % MOD
        }) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::super_pow(2, vec![3]), 8);
    assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
    assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
}
