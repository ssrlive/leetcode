#![allow(dead_code)]

/*

// 1994. The Number of Good Subsets
// https://leetcode.com/problems/the-number-of-good-subsets/
// https://leetcode.cn/problems/the-number-of-good-subsets/
//
// Hard
//
// You are given an integer array nums. We call a subset of nums good if its product can be represented as a product of one or more distinct prime numbers.

    For example, if nums = [1, 2, 3, 4]:
        [2, 3], [1, 2, 3], and [1, 3] are good subsets with products 6 = 2*3, 6 = 2*3, and 3 = 3 respectively.
        [1, 4] and [4] are not good subsets with products 4 = 2*2 and 4 = 2*2 respectively.

Return the number of different good subsets in nums modulo 109 + 7.

A subset of nums is any array that can be obtained by deleting some (possibly none or all) elements from nums. Two subsets are different if and only if the chosen indices to delete are different.

Example 1:

Input: nums = [1,2,3,4]
Output: 6
Explanation: The good subsets are:
- [1,2]: product is 2, which is the product of distinct prime 2.
- [1,2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [1,3]: product is 3, which is the product of distinct prime 3.
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [3]: product is 3, which is the product of distinct prime 3.

Example 2:

Input: nums = [4,2,3,15]
Output: 5
Explanation: The good subsets are:
- [2]: product is 2, which is the product of distinct prime 2.
- [2,3]: product is 6, which is the product of distinct primes 2 and 3.
- [2,15]: product is 30, which is the product of distinct primes 2, 3, and 5.
- [3]: product is 3, which is the product of distinct prime 3.
- [15]: product is 15, which is the product of distinct primes 3 and 5.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 30
*/

struct Solution;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let mod_num = 1_000_000_007;
        let mask = vec![
            -1_i64, 0, 1, 2, -1, 4, 3, 8, -1, -1, 5, 16, -1, 32, 9, 6, -1, 64, -1, 128, -1, 10, 17, 256, -1, -1, 33,
            -1, -1, 512, 7,
        ];
        let mut dp = vec![0_i64; 1025];
        dp[0] = 1;
        let mut cnt = vec![0_i64; 31];
        for i in 0..nums.len() {
            cnt[nums[i] as usize] += 1;
        }
        for i in 2..=30 {
            if mask[i] > -1 {
                let mi = mask[i] as usize;
                for mj in 0..1024 {
                    if (mi & mj) == 0 {
                        dp[mi | mj] = (dp[mi | mj] + cnt[i] * dp[mj]) % mod_num;
                    }
                }
            }
        }
        let mut res = dp.iter().skip(1).fold(0, |s, n| (s + n) % mod_num);
        cnt[1] -= 1;
        while cnt[1] >= 0 {
            res = (res << 1) % mod_num;
            cnt[1] -= 1;
        }
        res as _
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![1, 2, 3, 4], 6),
        (vec![4, 2, 3, 15], 5),
    ];
    for (nums, res) in cases {
        assert_eq!(Solution::number_of_good_subsets(nums), res);
    }
}
