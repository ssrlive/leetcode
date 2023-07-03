#![allow(dead_code)]

// 2513. Minimize the Maximum of Two Arrays
// https://leetcode.com/problems/minimize-the-maximum-of-two-arrays/
// https://leetcode.cn/problems/minimize-the-maximum-of-two-arrays/
//
// We have two arrays arr1 and arr2 which are initially empty. You need to add positive integers to them such that they satisfy all the following conditions:
//
// arr1 contains uniqueCnt1 distinct positive integers, each of which is not divisible by divisor1.
// arr2 contains uniqueCnt2 distinct positive integers, each of which is not divisible by divisor2.
// No integer is present in both arr1 and arr2.
//
// Given divisor1, divisor2, uniqueCnt1, and uniqueCnt2, return the minimum possible maximum integer that can be present in either array.
//
// Example 1:
//
// Input: divisor1 = 2, divisor2 = 7, uniqueCnt1 = 1, uniqueCnt2 = 3
// Output: 4
// Explanation:
// We can distribute the first 4 natural numbers into arr1 and arr2.
// arr1 = [1] and arr2 = [2,3,4].
// We can see that both arrays satisfy all the conditions.
// Since the maximum value is 4, we return it.
//
// Example 2:
//
// Input: divisor1 = 3, divisor2 = 5, uniqueCnt1 = 2, uniqueCnt2 = 1
// Output: 3
// Explanation:
// Here arr1 = [1,2], and arr2 = [3] satisfy all conditions.
// Since the maximum value is 3, we return it.
//
// Example 3:
//
// Input: divisor1 = 2, divisor2 = 4, uniqueCnt1 = 8, uniqueCnt2 = 2
// Output: 15
// Explanation:
// Here, the final possible arrays can be arr1 = [1,3,5,7,9,11,13,15], and arr2 = [2,6].
// It can be shown that it is not possible to obtain a lower maximum satisfying all conditions.
//
// Constraints:
//
// - 2 <= divisor1, divisor2 <= 10^5
// - 1 <= uniqueCnt1, uniqueCnt2 < 10^9
// - 2 <= uniqueCnt1 + uniqueCnt2 <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        fn valid(divisor1: i64, divisor2: i64, unique_cnt1: i64, unique_cnt2: i64, mid: i64) -> bool {
            let (cnt1, cnt2) = (mid / divisor1, mid / divisor2);
            let d = gcd(divisor1, divisor2);
            let dbl = mid / (divisor1 * divisor2 / d);

            cnt1 + unique_cnt1 <= mid && cnt2 + unique_cnt2 <= mid && mid - dbl >= unique_cnt1 + unique_cnt2
        }

        fn gcd(a: i64, b: i64) -> i64 {
            if a < b {
                return gcd(b, a);
            }
            if a % b == 0 {
                return b;
            }
            gcd(b, a % b)
        }

        let (mut left, mut right) = (1, i32::MAX);

        while left < right {
            let mid = left + (right - left) / 2;
            if valid(divisor1 as i64, divisor2 as i64, unique_cnt1 as i64, unique_cnt2 as i64, mid as i64) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimize_set(2, 7, 1, 3), 4);
    assert_eq!(Solution::minimize_set(3, 5, 2, 1), 3);
    assert_eq!(Solution::minimize_set(2, 4, 8, 2), 15);
}
