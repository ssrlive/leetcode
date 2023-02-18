#![allow(dead_code)]

/*

// 1716. Calculate Money in Leetcode Bank
Easy
537
16
Companies

Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.

He starts by putting in $1 on Monday, the first day. Every day from Tuesday to Sunday, he will put in $1 more than the day before. On every subsequent Monday, he will put in $1 more than the previous Monday.

Given n, return the total amount of money he will have in the Leetcode bank at the end of the nth day.

Example 1:

Input: n = 4
Output: 10
Explanation: After the 4th day, the total is 1 + 2 + 3 + 4 = 10.

Example 2:

Input: n = 10
Output: 37
Explanation: After the 10th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2nd Monday, Hercy only puts in $2.

Example 3:

Input: n = 20
Output: 96
Explanation: After the 20th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.

Constraints:

    1 <= n <= 1000
*/

struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut total = 0;
        let mut week = 0;
        let mut day = 1;
        for _ in 0..n {
            total += day + week;
            day += 1;
            if day == 8 {
                day = 1;
                week += 1;
            }
        }
        total
    }
}

#[test]
fn test() {
    assert_eq!(Solution::total_money(4), 10);
    assert_eq!(Solution::total_money(10), 37);
    assert_eq!(Solution::total_money(20), 96);
}
