#![allow(dead_code)]

/*

// 2571. Minimum Operations to Reduce an Integer to 0
// https://leetcode.com/problems/minimum-operations-to-reduce-an-integer-to-0/
// https://leetcode.cn/problems/minimum-operations-to-reduce-an-integer-to-0/
//
// Easy
//
// You are given a positive integer n, you can do the following operation any number of times:

    Add or subtract a power of 2 from n.

Return the minimum number of operations to make n equal to 0.

A number x is power of 2 if x == 2i where i >= 0.

Example 1:

Input: n = 39
Output: 3
Explanation: We can do the following operations:
- Add 20 = 1 to n, so now n = 40.
- Subtract 23 = 8 from n, so now n = 32.
- Subtract 25 = 32 from n, so now n = 0.
It can be shown that 3 is the minimum number of operations we need to make n equal to 0.

Example 2:

Input: n = 54
Output: 3
Explanation: We can do the following operations:
- Add 21 = 2 to n, so now n = 56.
- Add 23 = 8 to n, so now n = 64.
- Subtract 26 = 64 from n, so now n = 0.
So the minimum number of operations is 3.

Constraints:

    1 <= n <= 10^5
*/

struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        fn _min_operations(n: i32) -> i32 {
            let mut x = 1;
            if n <= 0 {
                return 0;
            }
            if n == 1 {
                return 1;
            }
            while x * 2 < n {
                x *= 2;
            }
            let y = x * 2 - n;
            x = n - x;
            let pick = _min_operations(x);
            let not_pick = _min_operations(y);
            pick.min(not_pick) + 1
        }
        _min_operations(n)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_operations(39), 3);
    assert_eq!(Solution::min_operations(54), 3);
}
