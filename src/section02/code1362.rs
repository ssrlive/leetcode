#![allow(dead_code)]

// 1362. Closest Divisors
// https://leetcode.com/problems/closest-divisors/
// https://leetcode.cn/problems/closest-divisors/
//
// Medium
//
// Given an integer num, find the closest two integers in absolute difference whose product equals num + 1 or num + 2.
//
// Return the two integers in any order.
//
// Example 1:
//
// Input: num = 8
// Output: [3,3]
// Explanation: For num + 1 = 9, the closest divisors are 3 & 3, for num + 2 = 10,
// the closest divisors are 2 & 5, hence 3 & 3 is chosen.
//
// Example 2:
//
// Input: num = 123
// Output: [5,25]
//
// Example 3:
//
// Input: num = 999
// Output: [40,25]
//
// Constraints:
//
// -    1 <= num <= 10^9
//

struct Solution;

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        fn find_divisors(num: i32) -> Vec<i32> {
            let mut i = (num as f64).sqrt() as i32;
            while num % i != 0 {
                i -= 1;
            }
            vec![i, num / i]
        }

        let mut a = find_divisors(num + 1);
        let mut b = find_divisors(num + 2);
        if (a[0] - a[1]).abs() > (b[0] - b[1]).abs() {
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}

#[test]
fn test() {
    assert_eq!(Solution::closest_divisors(8), vec![3, 3]);
    assert_eq!(Solution::closest_divisors(123), vec![5, 25]);
    assert_eq!(Solution::closest_divisors(999), vec![25, 40]);
}
