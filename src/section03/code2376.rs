#![allow(dead_code)]

/*

// 2376. Count Special Integers
// https://leetcode.com/problems/count-special-integers/
// https://leetcode.cn/problems/count-special-integers/
//
// Hard
//
// We call a positive integer special if all of its digits are distinct.

Given a positive integer n, return the number of special integers that belong to the interval [1, n].

Example 1:

Input: n = 20
Output: 19
Explanation: All the integers from 1 to 20, except 11, are special. Thus, there are 19 special integers.

Example 2:

Input: n = 5
Output: 5
Explanation: All the integers from 1 to 5 are special.

Example 3:

Input: n = 135
Output: 110
Explanation: There are 110 integers from 1 to 135 that are special.
Some of the integers that are not special are: 22, 114, and 131.

Constraints:

    1 <= n <= 2 * 10^9
*/

struct Solution;

impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        fn f(i: usize, bm: usize, t: usize, dp: &mut Vec<Vec<Vec<i32>>>, m: usize, a: &Vec<char>) -> i32 {
            if i == m {
                return 1;
            }
            let mut ans = dp[i][bm][t];
            if ans != -1 {
                return ans;
            }
            ans = 0;
            if t == 1 {
                let mut st: usize = 0;
                if bm == 0 {
                    st = 1;
                }
                for j in st..((a[i] as usize) - ('0' as usize) + 1) {
                    if (bm & (1 << j)) == 0 && j == (a[i] as usize) - ('0' as usize) {
                        ans += f(i + 1, bm | (1 << j), 1, dp, m, a);
                    } else if (bm & (1 << j)) == 0 {
                        ans += f(i + 1, bm | (1 << j), 0, dp, m, a);
                    }
                }
            } else {
                let mut st: usize = 0;
                if bm == 0 {
                    st = 1;
                }
                for j in st..10 {
                    if (bm & (1 << j)) == 0 {
                        ans += f(i + 1, bm | (1 << j), 0, dp, m, a);
                    }
                }
            }
            dp[i][bm][t] = ans;
            ans
        }

        let s: String = n.to_string();
        let m = s.chars().count();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![vec![-1_i32; 2]; (1 << 10) + 1]; m];
        let mut ans: i32 = 0;
        for i in 0..m {
            if i == 0 {
                ans += f(i, 0, 1, &mut dp, m, &s);
            } else {
                ans += f(i, 0, 0, &mut dp, m, &s);
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_special_numbers(20), 19);
    assert_eq!(Solution::count_special_numbers(5), 5);
    assert_eq!(Solution::count_special_numbers(135), 110);
    assert_eq!(Solution::count_special_numbers(1000000000), 5611770);
}
