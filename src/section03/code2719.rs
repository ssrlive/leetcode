#![allow(dead_code)]

// 2719. Count of Integers
// https://leetcode.com/problems/count-of-integers/
// https://leetcode.cn/problems/count-of-integers/
//
// Hard
//
// You are given two numeric strings num1 and num2 and two integers max_sum and min_sum. We denote an integer x to be good if:
//
//     num1 <= x <= num2
//     min_sum <= digit_sum(x) <= max_sum.
//
// Return the number of good integers. Since the answer may be large, return it modulo 109 + 7.
//
// Note that digit_sum(x) denotes the sum of the digits of x.
//
// Example 1:
//
// Input: num1 = "1", num2 = "12", min_sum = 1, max_sum = 8
// Output: 11
// Explanation: There are 11 integers whose sum of digits lies between 1 and 8 are 1,2,3,4,5,6,7,8,10,11, and 12. Thus, we return 11.
//
// Example 2:
//
// Input: num1 = "1", num2 = "5", min_sum = 1, max_sum = 5
// Output: 5
// Explanation: The 5 integers whose sum of digits lies between 1 and 5 are 1,2,3,4, and 5. Thus, we return 5.
//
// Constraints:
//
//     1 <= num1 <= num2 <= 10^22
//     1 <= min_sum <= max_sum <= 400
//

struct Solution;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        use std::collections::BTreeMap;
        let mut _num1 = String::from("");
        let mut leading_zeros = num2.len() - num1.len();
        while leading_zeros > 0 {
            _num1.push('0');
            leading_zeros -= 1;
        }
        _num1.push_str(num1.as_str());
        let num1 = _num1;

        let mut cache: BTreeMap<(usize, bool, bool, i16), i64> = BTreeMap::new();

        let num1 = num1.chars().map(|c| (c.to_digit(10).unwrap() as i16)).collect::<Vec<i16>>();
        let num2 = num2.chars().map(|c| (c.to_digit(10).unwrap() as i16)).collect::<Vec<i16>>();

        Self::solve(0, false, false, 0, &num1, &num2, min_sum as i16, max_sum as i16, &mut cache) as i32
    }

    #[allow(clippy::too_many_arguments)]
    fn solve(
        pos: usize,
        lower: bool,
        higher: bool,
        dig_sum: i16,
        num1: &Vec<i16>,
        num2: &Vec<i16>,
        min_sum: i16,
        max_sum: i16,
        cache: &mut std::collections::BTreeMap<(usize, bool, bool, i16), i64>,
    ) -> i64 {
        const MOD: i64 = 1000000007;
        if pos == num1.len() {
            if dig_sum >= min_sum && dig_sum <= max_sum {
                return 1;
            } else {
                return 0;
            }
        }

        if cache.contains_key(&(pos, lower, higher, dig_sum)) {
            return *cache.get(&(pos, lower, higher, dig_sum)).unwrap();
        }

        let lo = if !lower { num1[pos] } else { 0 };
        let hi = if !higher { num2[pos] } else { 9 };
        let mut count: i64 = 0;

        for idx in lo..=hi {
            if dig_sum + idx <= max_sum {
                count = (count
                    + Self::solve(
                        pos + 1,
                        lower || (idx > lo),
                        higher || (idx < hi),
                        dig_sum + idx,
                        num1,
                        num2,
                        min_sum,
                        max_sum,
                        cache,
                    ))
                    % MOD;
            }
        }

        cache.entry((pos, lower, higher, dig_sum)).or_insert(count);
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
    assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
    assert_eq!(Solution::count("0".to_string(), "9".to_string(), 1, 5), 5);
}
