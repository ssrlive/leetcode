#![allow(dead_code)]

/*

// 1862. Sum of Floored Pairs
// https://leetcode.com/problems/sum-of-floored-pairs/
// https://leetcode.cn/problems/sum-of-floored-pairs/
//
// Hard
//
// Given an integer array nums, return the sum of floor(nums[i] / nums[j]) for all pairs of indices 0 <= i, j < nums.length in the array. Since the answer may be too large, return it modulo 109 + 7.

The floor() function returns the integer part of the division.

Example 1:

Input: nums = [2,5,9]
Output: 10
Explanation:
floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
floor(5 / 2) = 2
floor(9 / 2) = 4
floor(9 / 5) = 1
We calculate the floor of the division for every pair of indices in the array then sum them up.

Example 2:

Input: nums = [7,7,7,7,7,7,7]
Output: 49

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::*;

        type Target = usize;
        type UseValue = usize;
        fn upper_bound(arr: &[Target], x: &UseValue) -> usize {
            let mut low = 0;
            let mut high = arr.len();
            while low != high {
                let mid = (low + high) / 2;
                match arr[mid].cmp(x) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        low = mid + 1;
                    }
                    std::cmp::Ordering::Greater => {
                        high = mid;
                    }
                }
            }
            low
        }

        fn lower_bound(arr: &[Target], x: &UseValue) -> usize {
            let mut low = 0;
            let mut high = arr.len();
            while low != high {
                let mid = (low + high) / 2;
                match arr[mid].cmp(x) {
                    std::cmp::Ordering::Less => {
                        low = mid + 1;
                    }
                    std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                        high = mid;
                    }
                }
            }
            low
        }

        const MOD: usize = 1_000_000_007;
        let mut map = HashMap::new();
        for &v in &nums {
            *map.entry(v as usize).or_insert(0) += 1;
        }

        let mut nums = nums.into_iter().map(|v| v as usize).collect::<Vec<usize>>();
        nums.sort();
        let n = nums.len();
        let mut result = 0;

        let mut memo: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let base = nums[i];
            if let Some(v) = memo.get(&base) {
                result += v;
                result %= MOD;
                continue;
            }

            let mut li = upper_bound(&nums, &base);
            let mut tot = 0;
            while li < n {
                let a = nums[li] / base;
                let v = (a + 1) * base;
                let ri = lower_bound(&nums, &v);

                let nv = ((ri - li) * a) % MOD;
                tot += nv;
                tot %= MOD;

                li = ri;
            }
            result += tot;
            result %= MOD;
            memo.insert(base, tot);
        }

        for (_, num) in map {
            result += num * num;
            result %= MOD;
        }

        result as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![2, 5, 9], 10),
        (vec![7, 7, 7, 7, 7, 7, 7], 49),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::sum_of_floored_pairs(nums), expected);
    }
}
