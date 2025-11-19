#![allow(dead_code)]

// 3677. Count Binary Palindromic Numbers
// https://leetcode.com/problems/count-binary-palindromic-numbers/
// https://leetcode.cn/problems/count-binary-palindromic-numbers/
//
// Hard
//
// You are given a non-negative integer n.
//
// A non-negative integer is called binary-palindromic if its binary representation (written without leading zeros) reads the same forward and backward.
//
// Return the number of integers k such that 0 <= k <= n and the binary representation of k is a palindrome.
//
// Note: The number 0 is considered binary-palindromic, and its representation is "0".
//
// Example 1:
//
// Input: n = 9
//
// Output: 6
//
// Explanation:
//
// The integers k in the range [0, 9] whose binary representations are palindromes are:
//
// 0 → "0"
// 1 → "1"
// 3 → "11"
// 5 → "101"
// 7 → "111"
// 9 → "1001"
// All other values in [0, 9] have non-palindromic binary forms. Therefore, the count is 6.
//
// Example 2:
//
// Input: n = 0
//
// Output: 1
//
// Explanation:
//
// Since "0" is a palindrome, the count is 1.
//
// Constraints:
//
// 0 <= n <= 10^15
//

struct Solution;

impl Solution {
    pub fn count_binary_palindromes(n: i64) -> i32 {
        fn make_pal(seed: u64, odd: bool) -> u64 {
            let mut res = seed;
            let mut t = seed;
            if odd {
                t >>= 1;
            }
            while t > 0 {
                res = (res << 1) | (t & 1);
                t >>= 1;
            }
            res
        }

        let nn = n as u64;

        if nn == 0 {
            return 1;
        }

        let mut ans: u64 = 1;
        let ll: u32 = 64 - nn.leading_zeros();

        for l in 1..ll {
            let h = l.div_ceil(2);
            ans += 1u64 << (h - 1);
        }

        let h = ll.div_ceil(2);
        let start: u64 = 1u64 << (h - 1);

        let shift = ll - h;
        let seed_n: u64 = if shift == 0 { nn } else { nn >> shift };

        if seed_n > start {
            ans += seed_n - start;
        }

        let p = make_pal(seed_n, (ll % 2) == 1);
        if p <= nn {
            ans += 1;
        }

        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_binary_palindromes(9), 6);
    assert_eq!(Solution::count_binary_palindromes(0), 1);
}
