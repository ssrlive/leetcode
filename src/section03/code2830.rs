#![allow(dead_code)]

// 2830. Maximize the Profit as the Salesman
// https://leetcode.com/problems/maximize-the-profit-as-the-salesman/
// https://leetcode.cn/problems/maximize-the-profit-as-the-salesman/
//
// Medium
//
// You are given an integer n representing the number of houses on a number line, numbered from 0 to n - 1.
//
// Additionally, you are given a 2D integer array offers where offers[i] = [starti, endi, goldi],
// indicating that ith buyer wants to buy all the houses from starti to endi for goldi amount of gold.
//
// As a salesman, your goal is to maximize your earnings by strategically selecting and selling houses to buyers.
//
// Return the maximum amount of gold you can earn.
//
// Note that different buyers can't buy the same house, and some houses may remain unsold.
//
// Example 1:
//
// Input: n = 5, offers = [[0,0,1],[0,2,2],[1,3,2]]
// Output: 3
// Explanation: There are 5 houses numbered from 0 to 4 and there are 3 purchase offers.
// We sell houses in the range [0,0] to 1st buyer for 1 gold and houses in the range [1,3] to 3rd buyer for 2 golds.
// It can be proven that 3 is the maximum amount of gold we can achieve.
//
// Example 2:
//
// Input: n = 5, offers = [[0,0,1],[0,2,10],[1,3,2]]
// Output: 10
// Explanation: There are 5 houses numbered from 0 to 4 and there are 3 purchase offers.
// We sell houses in the range [0,2] to 2nd buyer for 10 golds.
// It can be proven that 10 is the maximum amount of gold we can achieve.
//
// Constraints:
//
// 1 <= n <= 10^5
// 1 <= offers.length <= 10^5
// offers[i].length == 3
// 0 <= starti <= endi <= n - 1
// 1 <= goldi <= 10^3
//

struct Solution;

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut mx = vec![0; n];
        let mut offers = offers;

        let mut j = 0;
        offers.sort_by(|a, b| a[1].cmp(&b[1]));
        for i in 0..n {
            if i > 0 {
                mx[i] = mx[i - 1];
            }

            while j < offers.len() && offers[j][1] == i as i32 {
                let mut t = offers[j][2];
                let k = offers[j][0] as usize;
                if k > 0 {
                    t += mx[k - 1];
                }
                mx[i] = mx[i].max(t);
                j += 1;
            }
        }

        mx.into_iter().max().unwrap()
    }
}

#[test]
fn test() {
    let n = 5;
    let offers = vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]];
    let res = 3;
    assert_eq!(Solution::maximize_the_profit(n, offers), res);

    let n = 5;
    let offers = vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]];
    let res = 10;
    assert_eq!(Solution::maximize_the_profit(n, offers), res);
}
