#![allow(dead_code)]

// 2801. Count Stepping Numbers in Range
// https://leetcode.com/problems/count-stepping-numbers-in-range/
// https://leetcode.cn/problems/count-stepping-numbers-in-range/
//
// Hard
//
// Given two positive integers low and high represented as strings, find the count of stepping numbers in the inclusive range [low, high].
//
// A stepping number is an integer such that all of its adjacent digits have an absolute difference of exactly 1.
//
// Return an integer denoting the count of stepping numbers in the inclusive range [low, high].
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Note: A stepping number should not have a leading zero.
//
// Example 1:
//
// Input: low = "1", high = "11"
// Output: 10
// Explanation: The stepping numbers in the range [1,11] are 1, 2, 3, 4, 5, 6, 7, 8, 9 and 10.
// There are a total of 10 stepping numbers in the range. Hence, the output is 10.
//
// Example 2:
//
// Input: low = "90", high = "101"
// Output: 2
// Explanation: The stepping numbers in the range [90,101] are 98 and 101.
// There are a total of 2 stepping numbers in the range. Hence, the output is 2.
//
// Constraints:
//
// 1 <= int(low) <= int(high) < 10^100
// 1 <= low.length, high.length <= 100
// low and high consist of only digits.
// low and high don't have any leading zeros.
//

struct Solution;

struct Helper {
    s: Vec<usize>,
    dp: Vec<Vec<Vec<Vec<i32>>>>,
}

impl Helper {
    fn new(s: Vec<usize>, n: usize) -> Self {
        Self {
            s,
            dp: vec![vec![vec![vec![-1; 2]; 2]; 10]; n],
        }
    }

    fn eval(&mut self, i: usize, d: usize, low: usize, up: usize) -> i32 {
        if i == self.s.len() {
            return if low == 1 { 1 } else { 0 };
        }
        if self.dp[i][d][low][up] != -1 {
            return self.dp[i][d][low][up];
        }

        let mut ret = 0;
        for k in 0..10 {
            if up == 0 && k > self.s[i] {
                continue;
            }
            if low == 1 && i32::abs(k as i32 - d as i32) != 1 {
                continue;
            }

            let up_new = if k != self.s[i] { 1 } else { up };
            let low_new = if k > 0 { 1 } else { low };
            ret = (ret + self.eval(i + 1, k, low_new, up_new)) % 1_000_000_007;
        }
        self.dp[i][d][low][up] = ret;
        ret
    }
}

impl Solution {
    pub fn count_stepping_numbers(low: String, high: String) -> i32 {
        let low = low.chars().map(|c| c as usize - '0' as usize).collect::<Vec<_>>();
        let high = high.chars().map(|c| c as usize - '0' as usize).collect::<Vec<_>>();
        let (n, mut ret) = (high.len(), Self::extra(&low));
        let (mut s, mut t) = (Helper::new(low, n), Helper::new(high, n));

        ret += t.eval(0, 0, 0, 0) - s.eval(0, 0, 0, 0);
        if ret < 0 {
            ret += 1_000_000_007;
        }
        ret % 1_000_000_007
    }

    fn extra(nums: &[usize]) -> i32 {
        for i in 1..nums.len() {
            if i32::abs(nums[i - 1] as i32 - nums[i] as i32) != 1 {
                return 0;
            }
        }
        1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_stepping_numbers("1".to_string(), "11".to_string()), 10);
    assert_eq!(Solution::count_stepping_numbers("90".to_string(), "101".to_string()), 2);
}
