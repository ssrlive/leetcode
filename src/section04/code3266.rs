#![allow(dead_code)]

// 3266. Final Array State After K Multiplication Operations II
// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-ii/
// https://leetcode.cn/problems/final-array-state-after-k-multiplication-operations-ii/
//
// Hard
//
// You are given an integer array nums, an integer k, and an integer multiplier.
//
// You need to perform k operations on nums. In each operation:
//
//     Find the minimum value x in nums. If there are multiple occurrences of the minimum value, select the one that appears first.
//     Replace the selected minimum value x with x * multiplier.
//
// After the k operations, apply modulo 109 + 7 to every value in nums.
//
// Return an integer array denoting the final state of nums after performing all k operations and then applying the modulo.
//
// Example 1:
//
// Input: nums = [2,1,3,5,6], k = 5, multiplier = 2
//
// Output: [8,4,6,5,6]
//
// Explanation:
// Operation	Result
// After operation 1	[2, 2, 3, 5, 6]
// After operation 2	[4, 2, 3, 5, 6]
// After operation 3	[4, 4, 3, 5, 6]
// After operation 4	[4, 4, 6, 5, 6]
// After operation 5	[8, 4, 6, 5, 6]
// After applying modulo	[8, 4, 6, 5, 6]
//
// Example 2:
//
// Input: nums = [100000,2000], k = 2, multiplier = 1000000
//
// Output: [999999307,999999993]
//
// Explanation:
// Operation	Result
// After operation 1	[100000, 2000000000]
// After operation 2	[100000000000, 2000000000]
// After applying modulo	[999999307, 999999993]
//
// Constraints:
//
//     1 <= nums.length <= 10^4
//     1 <= nums[i] <= 10^9
//     1 <= k <= 10^9
//     1 <= multiplier <= 10^6
//

struct Solution;

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        let n = nums.len();
        if multiplier == 1 {
            return nums;
        }
        let multiplier = multiplier as i64;

        // Add everything to a priority queue (min-heap)
        let mut pq: std::collections::BinaryHeap<Reverse<_>> = nums
            .iter()
            .copied()
            .map(|x| x as i64)
            .enumerate()
            .map(|(i, x)| Reverse((x, i)))
            .collect();

        let mut multiplications = 0;
        let mut seen_count = 0;
        let mut seen = vec![false; n];
        while multiplications < k {
            // Dequeue
            let Reverse((x, i)) = pq.pop().unwrap();

            // Keep track of 'seen' elements
            if !seen[i] {
                seen[i] = true;
                seen_count += 1;
            }

            // Everything is now in an 'identical' state
            if x * multiplier > Self::MOD || seen_count == n {
                pq.push(Reverse((x, i)));
                break;
            }

            // Multiply and re-add!
            pq.push(Reverse((x * multiplier, i)));
            multiplications += 1;
        }

        // Deal with remaining multiplications
        let doubleups = (k - multiplications) % n as i32;
        for _ in 0..doubleups {
            let Reverse((x, i)) = pq.pop().unwrap();
            pq.push(Reverse((x * multiplier, i)));
        }

        let remaining = (k - multiplications) / n as i32;
        let multiplier = Self::pow(multiplier, remaining as i64);

        let mut nums = vec![0; n];
        for Reverse((x, i)) in pq {
            nums[i] = ((x % Self::MOD * multiplier) % Self::MOD) as i32;
        }
        nums
    }

    fn pow(mut multiplier: i64, mut exponent: i64) -> i64 {
        if exponent == 0 {
            return 1;
        }

        let mut ans = 1;
        multiplier %= Self::MOD;

        while exponent > 0 {
            if exponent % 2 == 1 {
                ans = (ans * multiplier) % Self::MOD;
            }

            exponent /= 2;
            multiplier = (multiplier * multiplier) % Self::MOD;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
    assert_eq!(
        Solution::get_final_state(vec![100000, 2000], 2, 1000000),
        vec![999999307, 999999993]
    );
}
