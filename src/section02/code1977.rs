#![allow(dead_code)]

/*

// 1977. Number of Ways to Separate Numbers
// https://leetcode.com/problems/number-of-ways-to-separate-numbers/
// https://leetcode.cn/problems/number-of-ways-to-separate-numbers/
//
// Hard
//
// You wrote down many positive integers in a string called num. However, you realized that you forgot to add commas to seperate the different numbers. You remember that the list of integers was non-decreasing and that no integer had leading zeros.

Return the number of possible lists of integers that you could have written down to get the string num. Since the answer may be large, return it modulo 109 + 7.

Example 1:

Input: num = "327"
Output: 2
Explanation: You could have written down the numbers:
3, 27
327

Example 2:

Input: num = "094"
Output: 0
Explanation: No numbers can have leading zeros and all numbers must be positive.

Example 3:

Input: num = "0"
Output: 0
Explanation: No numbers can have leading zeros and all numbers must be positive.

Constraints:

    1 <= num.length <= 3500
    num consists of digits '0' through '9'.
*/

struct Solution;

impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let num = num.as_bytes();
        let n = num.len();
        let mut a = vec![vec![0; n]; n];
        let mut b = vec![vec![0; n]; n];
        let mod_num = 1_000_000_007;
        for i in (0..n).rev() {
            if num[i] == b'0' {
                continue;
            }
            let mut sum = 0;
            for j in (i..n).rev() {
                if j == n - 1 {
                    a[i][j] = 1;
                } else {
                    let len = j - i + 1;
                    let first = j + 1;
                    let last = first + len;
                    let mut cnt = 0;
                    if last <= n && num[i..=j] <= num[first..last] {
                        cnt = (cnt + a[first][last - 1]) % mod_num;
                    }
                    if last < n {
                        cnt = (cnt + b[first][last]) % mod_num;
                    }
                    a[i][j] = cnt;
                }
                sum = (sum + a[i][j]) % mod_num;
                b[i][j] = sum;
            }
        }
        b[0][0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_combinations("327".to_string()), 2);
    assert_eq!(Solution::number_of_combinations("094".to_string()), 0);
    assert_eq!(Solution::number_of_combinations("0".to_string()), 0);
    assert_eq!(Solution::number_of_combinations("1234567890".to_string()), 41);
}
