#![allow(dead_code)]

/*

// 2234. Maximum Total Beauty of the Gardens
// https://leetcode.com/problems/maximum-total-beauty-of-the-gardens/
// https://leetcode.cn/problems/maximum-total-beauty-of-the-gardens/
//
// Hard
//
// Alice is a caretaker of n gardens and she wants to plant flowers to maximize the total beauty of all her gardens.

You are given a 0-indexed integer array flowers of size n, where flowers[i] is the number of flowers already planted in the ith garden. Flowers that are already planted cannot be removed. You are then given another integer newFlowers, which is the maximum number of flowers that Alice can additionally plant. You are also given the integers target, full, and partial.

A garden is considered complete if it has at least target flowers. The total beauty of the gardens is then determined as the sum of the following:

    The number of complete gardens multiplied by full.
    The minimum number of flowers in any of the incomplete gardens multiplied by partial. If there are no incomplete gardens, then this value will be 0.

Return the maximum total beauty that Alice can obtain after planting at most newFlowers flowers.

Example 1:

Input: flowers = [1,3,1,1], newFlowers = 7, target = 6, full = 12, partial = 1
Output: 14
Explanation: Alice can plant
- 2 flowers in the 0th garden
- 3 flowers in the 1st garden
- 1 flower in the 2nd garden
- 1 flower in the 3rd garden
The gardens will then be [3,6,2,2]. She planted a total of 2 + 3 + 1 + 1 = 7 flowers.
There is 1 garden that is complete.
The minimum number of flowers in the incomplete gardens is 2.
Thus, the total beauty is 1 * 12 + 2 * 1 = 12 + 2 = 14.
No other way of planting flowers can obtain a total beauty higher than 14.

Example 2:

Input: flowers = [2,4,5,3], newFlowers = 10, target = 5, full = 2, partial = 6
Output: 30
Explanation: Alice can plant
- 3 flowers in the 0th garden
- 0 flowers in the 1st garden
- 0 flowers in the 2nd garden
- 2 flowers in the 3rd garden
The gardens will then be [5,4,5,5]. She planted a total of 3 + 0 + 0 + 2 = 5 flowers.
There are 3 gardens that are complete.
The minimum number of flowers in the incomplete gardens is 4.
Thus, the total beauty is 3 * 2 + 4 * 6 = 6 + 24 = 30.
No other way of planting flowers can obtain a total beauty higher than 30.
Note that Alice could make all the gardens complete but in this case, she would obtain a lower total beauty.

Constraints:

    1 <= flowers.length <= 10^5
    1 <= flowers[i], target <= 10^5
    1 <= newFlowers <= 10^10
    1 <= full, partial <= 10^5
*/

struct Solution;

use std::cmp::max;
use std::cmp::min;

pub fn upper_bound<T: PartialOrd>(arr: &[T], target: T) -> usize {
    let mut l = 0;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
        let v = unsafe { arr.get_unchecked(m) };
        if *v <= target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

pub fn lower_bound<T: PartialOrd>(arr: &[T], target: T) -> usize {
    let mut l = 0;
    let mut r = arr.len();
    while l < r {
        let m = (l + r) / 2;
        let v = unsafe { arr.get_unchecked(m) };
        if *v >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

impl Solution {
    pub fn maximum_beauty(mut flowers: Vec<i32>, new_flowers: i64, target: i32, full: i32, partial: i32) -> i64 {
        flowers.sort();
        let ll = flowers.len();
        let mut arr = vec![0i64; ll];
        let mut prev_v = 0i64;
        for (i, v) in flowers.iter().enumerate() {
            arr[i] = i as i64 * (*v as i64 - prev_v);
            prev_v = *v as i64;
            if i > 0 {
                arr[i] += arr[i - 1];
            }
        }
        let mut best = 0i64;
        let mut left = new_flowers;
        for i in (0..=ll).rev() {
            if i < ll {
                let v = flowers[i];
                let need_to_target = max(target - v, 0i32);
                if need_to_target as i64 <= left {
                    left -= need_to_target as i64;
                } else {
                    break;
                }
            }
            let mut num_incomplete = upper_bound(&arr[..i], left);
            let bound_idx = lower_bound(&flowers[..num_incomplete], target);
            let found = if bound_idx > 0 {
                debug_assert!(flowers[bound_idx - 1] < target);
                num_incomplete = bound_idx;
                true
            } else {
                false
            };
            let min_flowers = if !found {
                0i64
            } else {
                let idx = num_incomplete - 1;
                let accum = arr[idx];
                let remain = left - accum;
                let distribute_remain = remain / num_incomplete as i64;
                let min_flowers = flowers[idx] as i64 + distribute_remain;
                min(min_flowers, target as i64 - 1i64)
            };
            let num_complete = if i == ll { 0 } else { ll - i };
            let score = num_complete as i64 * full as i64 + min_flowers * partial as i64;
            best = max(best, score);
        }
        best
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![1, 3, 1, 1], 7, 6, 12, 1, 14),
        (vec![2, 4, 5, 3], 10, 5, 2, 6, 30),
    ];
    for (f, n, t, full, p, ex) in cases {
        assert_eq!(Solution::maximum_beauty(f, n, t, full, p), ex);
    }
}
