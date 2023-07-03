#![allow(dead_code)]

/*

// 2233. Maximum Product After K Increments
// https://leetcode.com/problems/maximum-product-after-k-increments/
// https://leetcode.cn/problems/maximum-product-after-k-increments/
//
// Medium
//
// You are given an array of non-negative integers nums and an integer k. In one operation, you may choose any element from nums and increment it by 1.

Return the maximum product of nums after at most k operations. Since the answer may be very large, return it modulo 109 + 7. Note that you should maximize the product before taking the modulo.

Example 1:

Input: nums = [0,4], k = 5
Output: 20
Explanation: Increment the first number 5 times.
Now nums = [5, 4], with a product of 5 * 4 = 20.
It can be shown that 20 is maximum product possible, so we return 20.
Note that there may be other ways to increment nums to have the maximum product.

Example 2:

Input: nums = [6,3,3,2], k = 2
Output: 216
Explanation: Increment the second number 1 time and increment the fourth number 1 time.
Now nums = [6, 4, 3, 3], with a product of 6 * 4 * 3 * 3 = 216.
It can be shown that 216 is maximum product possible, so we return 216.
Note that there may be other ways to increment nums to have the maximum product.

Constraints:

    1 <= nums.length, k <= 10^5
    0 <= nums[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k as i64;
        let mut heap = std::collections::BinaryHeap::with_capacity(nums.len());
        nums.iter().for_each(|&x| heap.push(-x as i64));
        match heap.len() == 1 {
            true => *heap.peek_mut().unwrap() -= k,
            false => {
                while k > 0 {
                    let x = heap.pop().unwrap();
                    let next = *heap.peek().unwrap();
                    let delta = 1.max(x - next).min(k);
                    heap.push(x - delta);
                    k -= delta;
                }
            }
        }
        let sign = if nums.len() % 2 == 0 { 1 } else { -1 };
        (heap.iter().fold(1, |prod, &x| (prod * x) % 1_000_000_007) * sign) as _
    }
}

#[test]
fn test() {
    let cases = vec![(vec![0, 4], 5, 20), (vec![6, 3, 3, 2], 2, 216), (vec![1, 2, 3, 4], 4, 144)];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::maximum_product(nums, k), expected);
    }
}
