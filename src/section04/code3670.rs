#![allow(dead_code)]

// 3670. Maximum Product of Two Integers With No Common Bits
// https://leetcode.com/problems/maximum-product-of-two-integers-with-no-common-bits/
// https://leetcode.cn/problems/maximum-product-of-two-integers-with-no-common-bits
//
// Medium
//
// You are given an integer array nums.
//
// Your task is to find two distinct indices i and j such that the product nums[i] * nums[j] is maximized,
// and the binary representations of nums[i] and nums[j] do not share any common set bits.
//
// Return the maximum possible product of such a pair. If no such pair exists, return 0.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5,6,7]
//
// Output: 12
//
// Explanation:
//
// The best pair is 3 (011) and 4 (100). They share no set bits and 3 * 4 = 12.
//
// Example 2:
//
// Input: nums = [5,6,4]
//
// Output: 0
//
// Explanation:
//
// Every pair of numbers has at least one common set bit. Hence, the answer is 0.
//
// Example 3:
//
// Input: nums = [64,8,32]
//
// Output: 2048
//
// Explanation:
//
// No pair of numbers share a common bit, so the answer is the product of the two maximum elements, 64 and 32 (64 * 32 = 2048).
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        if nums.len() < 2 {
            return 0;
        }
        // Determine number of bits needed (max bit position among nums)
        let max_num = *nums.iter().max().unwrap() as u32;
        let bits = 32 - max_num.leading_zeros(); // up to 20 for constraint 1e6
        let b = bits.max(1); // ensure at least 1
        let size = 1usize << b;

        // Precompute masks for each number
        let mut masks: Vec<u32> = Vec::with_capacity(nums.len());
        for &num in &nums {
            let mut v = num as u32;
            let mut mask = 0u32;
            let mut bit = 0u32;
            while v > 0 {
                if v & 1 != 0 {
                    mask |= 1 << bit;
                }
                v >>= 1;
                bit += 1;
            }
            masks.push(mask);
        }

        // best[mask] stores the maximum value among numbers with EXACT mask initially.
        let mut best = vec![0i64; size];
        for (&num, &m) in nums.iter().zip(masks.iter()) {
            let idx = m as usize;
            if num as i64 > best[idx] {
                best[idx] = num as i64;
            }
        }

        // SOS DP to allow best[mask] to become the maximum among all submasks of mask.
        for bit in 0..b {
            let step = 1usize << bit;
            for mask in 0..size {
                if mask & step != 0 {
                    let without = mask ^ step;
                    if best[without] > best[mask] {
                        best[mask] = best[without];
                    }
                }
            }
        }

        let full_mask = (1u32 << b) - 1;
        let mut ans = 0i64;
        for (i, &num) in nums.iter().enumerate() {
            let m = masks[i];
            let complement = (full_mask ^ m) as usize;
            let other = best[complement];
            if other > 0 {
                // there exists a disjoint number
                let prod = other * num as i64;
                if prod > ans {
                    ans = prod;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4, 5, 6, 7]), 12);
    assert_eq!(Solution::max_product(vec![5, 6, 4]), 0);
    assert_eq!(Solution::max_product(vec![64, 8, 32]), 2048);
}
