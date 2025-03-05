#![allow(dead_code)]

// 793. Preimage Size of Factorial Zeroes Function
// https://leetcode.com/problems/preimage-size-of-factorial-zeroes-function/
// https://leetcode.cn/problems/preimage-size-of-factorial-zeroes-function/
//
// Let f(x) be the number of zeroes at the end of x!. Recall that x! = 1 * 2 * 3 * ... * x and by convention, 0! = 1.
//
// For example, f(3) = 0 because 3! = 6 has no zeroes at the end, while f(11) = 2 because 11! = 39916800 has two zeroes at the end.
// Given an integer k, return the number of non-negative integers x have the property that f(x) = k.
//
// Example 1:
//
// Input: k = 0
// Output: 5
// Explanation: 0!, 1!, 2!, 3!, and 4! end with k = 0 zeroes.
//
// Example 2:
//
// Input: k = 5
// Output: 0
// Explanation: There is no x such that x! ends in k = 5 zeroes.
//
// Example 3:
//
// Input: k = 3
// Output: 5
//
// Constraints:
//
// - 0 <= k <= 10^9
//

struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        fn trailing_zeros(n: i64) -> i32 {
            let mut n = n;
            let mut answer = 0;
            while n > 0 {
                n /= 5;
                answer += n as i32;
            }
            answer
        }

        let mut lo = 0;
        let mut hi = 5 * (k as i64);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let z = trailing_zeros(mid);
            if z < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if trailing_zeros(lo) == k { 5 } else { 0 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::preimage_size_fzf(0), 5);
    assert_eq!(Solution::preimage_size_fzf(5), 0);
    assert_eq!(Solution::preimage_size_fzf(3), 5);
}
