#![allow(dead_code)]

// 3233. Find the Count of Numbers Which Are Not Special
// https://leetcode.com/problems/find-the-count-of-numbers-which-are-not-special/
// https://leetcode.cn/problems/find-the-count-of-numbers-which-are-not-special/
//
// Medium
//
// You are given 2 positive integers l and r. For any number x,
// all positive divisors of x except x are called the proper divisors of x.
//
// A number is called special if it has exactly 2 proper divisors. For example:
//
//     The number 4 is special because it has proper divisors 1 and 2.
//     The number 6 is not special because it has proper divisors 1, 2, and 3.
//
// Return the count of numbers in the range [l, r] that are not special.
//
// Example 1:
//
// Input: l = 5, r = 7
//
// Output: 3
//
// Explanation:
//
// There are no special numbers in the range [5, 7].
//
// Example 2:
//
// Input: l = 4, r = 16
//
// Output: 11
//
// Explanation:
//
// The special numbers in the range [4, 16] are 4 and 9.
//
// Constraints:
//
//     1 <= l <= r <= 10^9
//

struct Solution;

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let lim = (r as f64).sqrt() as i32;
        let mut v = vec![true; (lim + 1) as usize];
        v[0] = false;
        v[1] = false;

        let mut i = 2;
        while i * i <= lim {
            if v[i as usize] {
                let mut j = i * i;
                while j <= lim {
                    v[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }

        let mut cnt = 0;
        for i in 2..=lim {
            if v[i as usize] {
                let square = i * i;
                if square >= l && square <= r {
                    cnt += 1;
                }
            }
        }

        let total_count = r - l + 1;
        total_count - cnt
    }
}

#[test]
fn test() {
    let l = 5;
    let r = 7;
    let expected = 3;
    assert_eq!(Solution::non_special_count(l, r), expected);

    let l = 4;
    let r = 16;
    let expected = 11;
    assert_eq!(Solution::non_special_count(l, r), expected);
}
