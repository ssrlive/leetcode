#![allow(dead_code)]

/*

// 2584. Split the Array to Make Coprime Products
// https://leetcode.com/problems/split-the-array-to-make-coprime-products/
// https://leetcode.cn/problems/split-the-array-to-make-coprime-products/
//
// Medium
//
// You are given a 0-indexed integer array nums of length n.

A split at an index i where 0 <= i <= n - 2 is called valid if the product of the first i + 1 elements and the product of the remaining elements are coprime.

    For example, if nums = [2, 3, 3], then a split at the index i = 0 is valid because 2 and 9 are coprime, while a split at the index i = 1 is not valid because 6 and 3 are not coprime. A split at the index i = 2 is not valid because i == n - 1.

Return the smallest index i at which the array can be split validly or -1 if there is no such split.

Two values val1 and val2 are coprime if gcd(val1, val2) == 1 where gcd(val1, val2) is the greatest common divisor of val1 and val2.

Example 1:

Input: nums = [4,7,8,15,3,5]
Output: 2
Explanation: The table above shows the values of the product of the first i + 1 elements, the remaining elements, and their gcd at each index i.
The only valid split is at index 2.

Example 2:

Input: nums = [4,7,15,8,3,5]
Output: -1
Explanation: The table above shows the values of the product of the first i + 1 elements, the remaining elements, and their gcd at each index i.
There is no valid split.

Constraints:

    n == nums.length
    1 <= n <= 10^4
    1 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn factorize(n: i32) -> Vec<i32> {
            let mut res = Vec::new();
            let mut n = n;
            let mut i = 2;
            while n > 1 && i < 1000 {
                if n % i == 0 {
                    res.push(i);
                    while n % i == 0 {
                        n /= i;
                    }
                }
                i += 1 + (i % 2);
            }
            if n > 1 {
                res.push(n);
            }
            res
        }

        let (mut left, mut right) = (HashMap::new(), HashMap::new());
        for n in nums.iter() {
            for f in factorize(*n) {
                *right.entry(f).or_insert(0) += 1;
            }
        }
        let mut common = 0;
        for (i, &num) in nums.iter().enumerate().take(nums.len() - 1) {
            for f in factorize(num) {
                let l = left.entry(f).or_insert(0);
                common += (*l == 0) as i32;
                *l += 1;
                common -= (*l == right[&f]) as i32;
            }
            if common == 0 {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![4, 7, 8, 15, 3, 5];
    let res = 2;
    assert_eq!(Solution::find_valid_split(nums), res);
    let nums = vec![4, 7, 15, 8, 3, 5];
    let res = -1;
    assert_eq!(Solution::find_valid_split(nums), res);
}
