#![allow(dead_code)]

// 2412. Minimum Money Required Before Transactions
// https://leetcode.com/problems/minimum-money-required-before-transactions/
// https://leetcode.cn/problems/minimum-money-required-before-transactions/
//
// You are given a 0-indexed 2D integer array transactions, where transactions[i] = [costi, cashbacki].
//
// The array describes transactions, where each transaction must be completed exactly once in some order.
// At any given moment, you have a certain amount of money. In order to complete transaction i, money >= costi must hold true.
// After performing a transaction, money becomes money - costi + cashbacki.
//
// Return the minimum amount of money required before any transaction so that all of the transactions
// can be completed regardless of the order of the transactions.
//
// Example 1:
//
// Input: transactions = [[2,1],[5,0],[4,2]]
// Output: 10
// Explanation:
// Starting with money = 10, the transactions can be performed in any order.
// It can be shown that starting with money < 10 will fail to complete all transactions in some order.
//
// Example 2:
//
// Input: transactions = [[3,0],[0,3]]
// Output: 3
// Explanation:
// - If transactions are in the order [[3,0],[0,3]], the minimum money required to complete the transactions is 3.
// - If transactions are in the order [[0,3],[3,0]], the minimum money required to complete the transactions is 0.
// Thus, starting with money = 3, the transactions can be performed in any order.
//
// Constraints:
//
// - 1 <= transactions.length <= 10^5
// - transactions[i].length == 2
// - 0 <= costi, cashbacki <= 10^9
//

struct Solution;

impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0;
        for t in &transactions {
            sum += i32::max(0, t[0] - t[1]) as i64;
        }

        let mut ret = sum;
        for t in transactions {
            if t[0] <= t[1] {
                ret = ret.max(sum + t[0] as i64);
            } else {
                ret = ret.max(sum + t[1] as i64);
            }
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]), 10);
    assert_eq!(Solution::minimum_money(vec![vec![3, 0], vec![0, 3]]), 3);
}
