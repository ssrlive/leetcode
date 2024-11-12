#![allow(dead_code)]

// 3352. Count K-Reducible Numbers Less Than N
// https://leetcode.com/problems/count-k-reducible-numbers-less-than-n/
// https://leetcode.cn/problems/count-k-reducible-numbers-less-than-n/
//
// Hard
//
// You are given a binary string s representing a number n in its binary form.
//
// You are also given an integer k.
//
// An integer x is called k-reducible if performing the following operation at most k times reduces it to 1:
//
//     Replace x with the count of
//     set bits
//     in its binary representation.
//
// For example, the binary representation of 6 is "110".
// Applying the operation once reduces it to 2 (since "110" has two set bits).
// Applying the operation again to 2 (binary "10") reduces it to 1 (since "10" has one set bit).
//
// Return an integer denoting the number of positive integers less than n that are k-reducible.
//
// Since the answer may be too large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "111", k = 1
//
// Output: 3
//
// Explanation:
//
// n = 7. The 1-reducible integers less than 7 are 1, 2, and 4.
//
// Example 2:
//
// Input: s = "1000", k = 2
//
// Output: 6
//
// Explanation:
//
// n = 8. The 2-reducible integers less than 8 are 1, 2, 3, 4, 5, and 6.
//
// Example 3:
//
// Input: s = "1", k = 3
//
// Output: 0
//
// Explanation:
//
// There are no positive integers less than n = 1, so the answer is 0.
//
// Constraints:
//
//     1 <= s.length <= 800
//     s has no leading zeros.
//     s consists only of the characters '0' and '1'.
//     1 <= k <= 5
//

struct Solution;

impl Solution {
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        use std::collections::{BTreeSet, HashMap};
        const MOD_NUM: i32 = 1_000_000_007;
        let n = s.len() as i32;

        fn set_bits_in(v: i32) -> i32 {
            let mut ans = 0;
            for bit in 0..32 {
                if (v >> bit) & 1 == 1 {
                    ans += 1;
                }
            }
            ans
        }

        // we can have at max 800 set bits
        // filter out the number of set bits we can have to reach 1 with atmax k operations
        let mut can_have_set_bits = BTreeSet::new();
        let mut t = HashMap::new();
        for i in 1..=n {
            t.insert(i, 1 + t.get(&set_bits_in(i)).unwrap_or(&0));
            if t[&i] <= k {
                can_have_set_bits.insert(i);
            }
        }

        let mut dp = vec![vec![vec![-1; 801]; 2]; 801];

        fn recur(dp: &mut [Vec<Vec<i32>>], s: &str, can_have_set_bits: &BTreeSet<i32>, pos: usize, tight: bool, set_bits: i32) -> i32 {
            let n = s.len();
            if pos == n {
                return if !tight && can_have_set_bits.contains(&set_bits) { 1 } else { 0 };
            }
            if set_bits > *can_have_set_bits.iter().next_back().unwrap() {
                return 0;
            }

            if dp[pos][tight as usize][set_bits as usize] != -1 {
                return dp[pos][tight as usize][set_bits as usize];
            }

            // add 0 here
            let tight0 = tight && (s.chars().nth(pos).unwrap() == '0');
            let mut ans = recur(dp, s, can_have_set_bits, pos + 1, tight0, set_bits);

            // add 1 here
            if !tight || s.chars().nth(pos).unwrap() == '1' {
                let tight1 = tight && (s.chars().nth(pos).unwrap() == '1');
                ans = (ans + recur(dp, s, can_have_set_bits, pos + 1, tight1, set_bits + 1)) % MOD_NUM;
            }
            dp[pos][tight as usize][set_bits as usize] = ans;
            ans
        }

        recur(&mut dp, &s, &can_have_set_bits, 0, true, 0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_k_reducible_numbers("111".to_string(), 1), 3);
    assert_eq!(Solution::count_k_reducible_numbers("1000".to_string(), 2), 6);
    assert_eq!(Solution::count_k_reducible_numbers("1".to_string(), 3), 0);
}
