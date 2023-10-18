#![allow(dead_code)]

// 2861. Maximum Number of Alloys
// https://leetcode.com/problems/maximum-number-of-alloys/
// https://leetcode.cn/problems/maximum-number-of-alloys/
//
// Medium
//
// You are the owner of a company that creates alloys using various types of metals.
// There are n different types of metals available, and you have access to k machines that can be used to create alloys.
// Each machine requires a specific amount of each metal type to create an alloy.
//
// For the ith machine to create an alloy, it needs composition[i][j] units of metal of type j.
// Initially, you have stock[i] units of metal type i, and purchasing one unit of metal type i costs cost[i] coins.
//
// Given integers n, k, budget, a 1-indexed 2D array composition, and 1-indexed arrays stock and cost,
// your goal is to maximize the number of alloys the company can create while staying within the budget of budget coins.
//
// All alloys must be created with the same machine.
//
// Return the maximum number of alloys that the company can create.
//
// Example 1:
//
// Input: n = 3, k = 2, budget = 15, composition = [[1,1,1],[1,1,10]], stock = [0,0,0], cost = [1,2,3]
// Output: 2
// Explanation: It is optimal to use the 1st machine to create alloys.
// To create 2 alloys we need to buy the:
// - 2 units of metal of the 1st type.
// - 2 units of metal of the 2nd type.
// - 2 units of metal of the 3rd type.
// In total, we need 2 * 1 + 2 * 2 + 2 * 3 = 12 coins, which is smaller than or equal to budget = 15.
// Notice that we have 0 units of metal of each type and we have to buy all the required units of metal.
// It can be proven that we can create at most 2 alloys.
//
// Example 2:
//
// Input: n = 3, k = 2, budget = 15, composition = [[1,1,1],[1,1,10]], stock = [0,0,100], cost = [1,2,3]
// Output: 5
// Explanation: It is optimal to use the 2nd machine to create alloys.
// To create 5 alloys we need to buy:
// - 5 units of metal of the 1st type.
// - 5 units of metal of the 2nd type.
// - 0 units of metal of the 3rd type.
// In total, we need 5 * 1 + 5 * 2 + 0 * 3 = 15 coins, which is smaller than or equal to budget = 15.
// It can be proven that we can create at most 5 alloys.
//
// Example 3:
//
// Input: n = 2, k = 3, budget = 10, composition = [[2,1],[1,2],[1,1]], stock = [1,1], cost = [5,5]
// Output: 2
// Explanation: It is optimal to use the 3rd machine to create alloys.
// To create 2 alloys we need to buy the:
// - 1 unit of metal of the 1st type.
// - 1 unit of metal of the 2nd type.
// In total, we need 1 * 5 + 1 * 5 = 10 coins, which is smaller than or equal to budget = 10.
// It can be proven that we can create at most 2 alloys.
//
// Constraints:
//
// 1 <= n, k <= 100
// 0 <= budget <= 10^8
// composition.length == k
// composition[i].length == n
// 1 <= composition[i][j] <= 100
// stock.length == cost.length == n
// 0 <= stock[i] <= 10^8
// 1 <= cost[i] <= 100
//

struct Solution;

impl Solution {
    pub fn max_number_of_alloys(_n: i32, _k: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, i32::MAX as i64);

        while left < right {
            let mid = right - (right - left) / 2;
            if Self::check(budget, &composition, &stock, &cost, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left as _
    }

    fn check(budget: i32, composition: &[Vec<i32>], stock: &[i32], cost: &[i32], mid: i64) -> bool {
        let (k, n) = (composition.len(), cost.len());
        for item in composition.iter().take(k) {
            let mut spend = 0;
            for j in 0..n {
                let extra = (mid * item[j] as i64 - stock[j] as i64).max(0);
                spend += extra * cost[j] as i64;
            }
            if spend > budget as i64 {
                continue;
            }
            return true;
        }
        false
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 2;
    let budget = 15;
    let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
    let stock = vec![0, 0, 0];
    let cost = vec![1, 2, 3];
    let res = 2;
    assert_eq!(Solution::max_number_of_alloys(n, k, budget, composition, stock, cost), res);

    let n = 3;
    let k = 2;
    let budget = 15;
    let composition = vec![vec![1, 1, 1], vec![1, 1, 10]];
    let stock = vec![0, 0, 100];
    let cost = vec![1, 2, 3];
    let res = 5;
    assert_eq!(Solution::max_number_of_alloys(n, k, budget, composition, stock, cost), res);

    let n = 2;
    let k = 3;
    let budget = 10;
    let composition = vec![vec![2, 1], vec![1, 2], vec![1, 1]];
    let stock = vec![1, 1];
    let cost = vec![5, 5];
    let res = 2;
    assert_eq!(Solution::max_number_of_alloys(n, k, budget, composition, stock, cost), res);
}
