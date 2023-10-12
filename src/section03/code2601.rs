#![allow(dead_code)]

/*
// 2601. Prime Subtraction Operation
// https://leetcode.com/problems/prime-subtraction-operation/
// https://leetcode.cn/problems/prime-subtraction-operation/
//
// Medium
//
// You are given a 0-indexed integer array nums of length n.

You can perform the following operation as many times as you want:

Pick an index i that you haven’t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].
Return true if you can make nums a strictly increasing array using the above operation and false otherwise.

A strictly increasing array is an array whose each element is strictly greater than its preceding element.

Example 1:

Input: nums = [4,9,6,10]
Output: true
Explanation: In the first operation: Pick i = 0 and p = 3, and then subtract 3 from nums[0], so that nums becomes [1,9,6,10].
In the second operation: i = 1, p = 7, subtract 7 from nums[1], so nums becomes equal to [1,2,6,10].
After the second operation, nums is sorted in strictly increasing order, so the answer is true.
Example 2:

Input: nums = [6,8,11,12]
Output: true
Explanation: Initially nums is sorted in strictly increasing order, so we don't need to make any operations.
Example 3:

Input: nums = [5,8,3]
Output: false
Explanation: It can be proven that there is no way to perform operations to make nums sorted in strictly increasing order, so the answer is false.

Constraints:

1 <= nums.length <= 1000
1 <= nums[i] <= 1000
nums.length == n
*/

struct Solution;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        use std::cmp::Ordering;
        fn lower_bound_by<T, F>(arr: &[T], f: F) -> Result<usize, usize>
        where
            T: Ord,
            F: Fn(&T) -> Ordering,
        {
            arr.iter().position(|y| f(y) != Ordering::Less).ok_or(arr.len())
        }

        fn lower_bound<T: Ord>(arr: &[T], x: &T) -> Result<usize, usize> {
            lower_bound_by(arr, |y| y.cmp(x))
        }

        fn get_primes() -> Vec<i32> {
            let mut ps = vec![2];
            let mut sieve = vec![false; 1001];
            for i in 3..32 {
                if !sieve[i] {
                    for j in (i * i..=1000).step_by(i) {
                        sieve[j] = true;
                    }
                }
            }
            for i in (3..=1000).step_by(2) {
                if !sieve[i as usize] {
                    ps.push(i);
                }
            }
            ps
        }

        let mut nums = nums;
        let ps = get_primes();
        for i in 0..nums.len() {
            let it = lower_bound(&ps, &(nums[i] - if i > 0 { nums[i - 1] } else { 0 }));
            let it = it.unwrap_or(ps.len());
            if it > 0 {
                nums[i] -= ps[it - 1];
            }
            if i > 0 && nums[i] <= nums[i - 1] {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    let nums = vec![4, 9, 6, 10];
    assert!(Solution::prime_sub_operation(nums));
    let nums = vec![6, 8, 11, 12];
    assert!(Solution::prime_sub_operation(nums));
    let nums = vec![5, 8, 3];
    assert!(!Solution::prime_sub_operation(nums));
}
