#![allow(dead_code)]

/*

// 2048. Next Greater Numerically Balanced Number
// https://leetcode.com/problems/next-greater-numerically-balanced-number/
// https://leetcode.cn/problems/next-greater-numerically-balanced-number/
//
// Medium
//
// An integer x is numerically balanced if for every digit d in the number x, there are exactly d occurrences of that digit in x.

Given an integer n, return the smallest numerically balanced number strictly greater than n.

Example 1:

Input: n = 1
Output: 22
Explanation:
22 is numerically balanced since:
- The digit 2 occurs 2 times.
It is also the smallest numerically balanced number strictly greater than 1.

Example 2:

Input: n = 1000
Output: 1333
Explanation:
1333 is numerically balanced since:
- The digit 1 occurs 1 time.
- The digit 3 occurs 3 times.
It is also the smallest numerically balanced number strictly greater than 1000.
Note that 1022 cannot be the answer because 0 appeared more than 0 times.

Example 3:

Input: n = 3000
Output: 3133
Explanation:
3133 is numerically balanced since:
- The digit 1 occurs 1 time.
- The digit 3 occurs 3 times.
It is also the smallest numerically balanced number strictly greater than 3000.

Constraints:

    0 <= n <= 10^6
*/

struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        #[rustfmt::skip]
        let numbers = vec![
            0, 1, 22, 122, 212, 221, 333, 1333, 3133, 3313, 3331, 4444, 14444, 22333, 23233, 23323,
            23332, 32233, 32323, 32332, 33223, 33232, 33322, 41444, 44144, 44414, 44441, 55555,
            122333, 123233, 123323, 123332, 132233, 132323, 132332, 133223, 133232, 133322, 155555,
            212333, 213233, 213323, 213332, 221333, 223133, 223313, 223331, 224444, 231233, 231323,
            231332, 232133, 232313, 232331, 233123, 233132, 233213, 233231, 233312, 233321, 242444,
            244244, 244424, 244442, 312233, 312323, 312332, 313223, 313232, 313322, 321233, 321323,
            321332, 322133, 322313, 322331, 323123, 323132, 323213, 323231, 323312, 323321, 331223,
            331232, 331322, 332123, 332132, 332213, 332231, 332312, 332321, 333122, 333212, 333221,
            422444, 424244, 424424, 424442, 442244, 442424, 442442, 444224, 444242, 444422, 515555,
            551555, 555155, 555515, 555551, 666666, 1224444,
        ];
        let mut l = 0;
        let mut r = numbers.len();
        let mut found = i32::MAX;
        while l <= r {
            let m = (l + r) / 2;
            let k = numbers[m];
            match k.cmp(&n) {
                std::cmp::Ordering::Equal => return numbers[m + 1],
                std::cmp::Ordering::Greater => {
                    found = k;
                    r = m - 1;
                }
                std::cmp::Ordering::Less => l = m + 1,
            }
        }
        found
    }
}

#[test]
fn test() {
    assert_eq!(Solution::next_beautiful_number(1), 22);
    assert_eq!(Solution::next_beautiful_number(1000), 1333);
    assert_eq!(Solution::next_beautiful_number(3000), 3133);
    assert_eq!(Solution::next_beautiful_number(122333), 123233);
}
