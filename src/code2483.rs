#![allow(dead_code)]

// 2483. Minimum Penalty for a Shop
// https://leetcode.com/problems/minimum-penalty-for-a-shop/
// https://leetcode.cn/problems/minimum-penalty-for-a-shop/
//
// You are given the customer visit log of a shop represented by a 0-indexed string customers consisting only of characters 'N' and 'Y':
//
// - if the ith character is 'Y', it means that customers come at the ith hour
// - whereas 'N' indicates that no customers come at the ith hour.
//
// If the shop closes at the jth hour (0 <= j <= n), the penalty is calculated as follows:
//
// - For every hour when the shop is open and no customers come, the penalty increases by 1.
// - For every hour when the shop is closed and customers come, the penalty increases by 1.
//
// Return the earliest hour at which the shop must be closed to incur a minimum penalty.
//
// Note that if a shop closes at the jth hour, it means the shop is closed at the hour j.
//
// Example 1:
//
// Input: customers = "YYNY"
// Output: 2
// Explanation:
// - Closing the shop at the 0th hour incurs in 1+1+0+1 = 3 penalty.
// - Closing the shop at the 1st hour incurs in 0+1+0+1 = 2 penalty.
// - Closing the shop at the 2nd hour incurs in 0+0+0+1 = 1 penalty.
// - Closing the shop at the 3rd hour incurs in 0+0+1+1 = 2 penalty.
// - Closing the shop at the 4th hour incurs in 0+0+1+0 = 1 penalty.
// Closing the shop at 2nd or 4th hour gives a minimum penalty. Since 2 is earlier, the optimal closing time is 2.
//
// Example 2:
//
// Input: customers = "NNNNN"
// Output: 0
// Explanation: It is best to close the shop at the 0th hour as no customers arrive.
//
// Example 3:
//
// Input: customers = "YYYY"
// Output: 4
// Explanation: It is best to close the shop at the 4th hour as customers arrive at each hour.
//
// Constraints:
//
// - 1 <= customers.length <= 10^5
// - customers consists only of characters 'Y' and 'N'.
//

struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut cnt_y = customers.chars().fold(0, |sum, c| if c == 'Y' { sum + 1 } else { sum });

        let f = |(min_pen, min_hour, mut cnt_n): (i32, i32, i32), (i, c): (usize, char)| {
            let curr_pen = cnt_n + cnt_y;
            if c == 'Y' {
                cnt_y -= 1;
            } else {
                cnt_n += 1;
            }

            if curr_pen < min_pen {
                (curr_pen, i as i32, cnt_n)
            } else {
                (min_pen, min_hour, cnt_n)
            }
        };
        let pen = customers.chars().enumerate().fold((i32::MAX, 0, 0), f);

        if pen.2 >= pen.0 {
            pen.1 as i32
        } else {
            customers.len() as i32
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
    assert_eq!(Solution::best_closing_time("NNNNN".to_string()), 0);
    assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
}
