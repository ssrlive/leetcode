#![allow(dead_code)]

// 2466. Count Ways To Build Good Strings
// https://leetcode.com/problems/count-ways-to-build-good-strings/
// https://leetcode.cn/problems/count-ways-to-build-good-strings/
//
// Given the integers zero, one, low, and high, we can construct a string by starting with an empty string,
// and then at each step perform either of the following:
//
// - Append the character '0' zero times.
// - Append the character '1' one times.
//
// This can be performed any number of times.
//
// A good string is a string constructed by the above process having a length between low and high (inclusive).
//
// Return the number of different good strings that can be constructed satisfying these properties.
// Since the answer can be large, return it modulo 10^9 + 7.
//
// Example 1:
//
// Input: low = 3, high = 3, zero = 1, one = 1
// Output: 8
// Explanation:
// One possible valid good string is "011".
// It can be constructed as follows: "" -> "0" -> "01" -> "011".
// All binary strings from "000" to "111" are good strings in this example.
//
// Example 2:
//
// Input: low = 2, high = 3, zero = 1, one = 2
// Output: 5
// Explanation: The good strings are "00", "11", "000", "110", and "011".
//
// Constraints:
//
// - 1 <= low <= high <= 10^5
// - 1 <= zero, one <= low
//

struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let low = low as usize;
        let high = high as usize;
        let one = one as usize;
        let zero = zero as usize;
        let mut ans = 0;
        let mut arr = vec![0; high + 1];
        arr[0] = 1;
        for i in 0..=high {
            if i >= zero {
                arr[i] = (arr[i] + arr[i - zero]) % MOD;
            }
            if i >= one {
                arr[i] = (arr[i] + arr[i - one]) % MOD;
            }
            if i >= low {
                ans = (ans + arr[i]) % MOD;
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
    assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
}
