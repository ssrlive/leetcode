#![allow(dead_code)]

// 3562. Maximum Profit from Trading Stocks with Discounts
// https://leetcode.com/problems/maximum-profit-from-trading-stocks-with-discounts/
// https://leetcode.cn/problems/maximum-profit-from-trading-stocks-with-discounts/
//
// Hard
//
// You are given an integer n, representing the number of employees in a company. Each employee is assigned a unique ID from 1 to n,
// and employee 1 is the CEO. You are given two 1-based integer arrays, present and future, each of length n, where:
//
//     present[i] represents the current price at which the ith employee can buy a stock today.
//     future[i] represents the expected price at which the ith employee can sell the stock tomorrow.
//
// The company's hierarchy is represented by a 2D integer array hierarchy, where hierarchy[i] = [ui, vi] means that employee ui is the direct boss of employee vi.
//
// Additionally, you have an integer budget representing the total funds available for investment.
//
// However, the company has a discount policy: if an employee's direct boss purchases their own stock,
// then the employee can buy their stock at half the original price (floor(present[v] / 2)).
//
// Return the maximum profit that can be achieved without exceeding the given budget.
//
// Note:
//
//     You may buy each stock at most once.
//     You cannot use any profit earned from future stock prices to fund additional investments and must buy only from budget.
//
// Example 1:
//
// Input: n = 2, present = [1,2], future = [4,3], hierarchy = [[1,2]], budget = 3
//
// Output: 5
//
// Explanation:
//
//     Employee 1 buys the stock at price 1 and earns a profit of 4 - 1 = 3.
//     Since Employee 1 is the direct boss of Employee 2, Employee 2 gets a discounted price of floor(2 / 2) = 1.
//     Employee 2 buys the stock at price 1 and earns a profit of 3 - 1 = 2.
//     The total buying cost is 1 + 1 = 2 <= budget. Thus, the maximum total profit achieved is 3 + 2 = 5.
//
// Example 2:
//
// Input: n = 2, present = [3,4], future = [5,8], hierarchy = [[1,2]], budget = 4
//
// Output: 4
//
// Explanation:
//
//     Employee 2 buys the stock at price 4 and earns a profit of 8 - 4 = 4.
//     Since both employees cannot buy together, the maximum profit is 4.
//
// Example 3:
//
// Input: n = 3, present = [4,6,8], future = [7,9,11], hierarchy = [[1,2],[1,3]], budget = 10
//
// Output: 10
//
// Explanation:
//
//     Employee 1 buys the stock at price 4 and earns a profit of 7 - 4 = 3.
//     Employee 3 would get a discounted price of floor(8 / 2) = 4 and earns a profit of 11 - 4 = 7.
//     Employee 1 and Employee 3 buy their stocks at a total cost of 4 + 4 = 8 <= budget. Thus, the maximum total profit achieved is 3 + 7 = 10.
//
// Example 4:
//
// Input: n = 3, present = [5,2,3], future = [8,5,6], hierarchy = [[1,2],[2,3]], budget = 7
//
// Output: 12
//
// Explanation:
//
//     Employee 1 buys the stock at price 5 and earns a profit of 8 - 5 = 3.
//     Employee 2 would get a discounted price of floor(2 / 2) = 1 and earns a profit of 5 - 1 = 4.
//     Employee 3 would get a discounted price of floor(3 / 2) = 1 and earns a profit of 6 - 1 = 5.
//     The total cost becomes 5 + 1 + 1 = 7 <= budget. Thus, the maximum total profit achieved is 3 + 4 + 5 = 12.
//
// Constraints:
//
//     1 <= n <= 160
//     present.length, future.length == n
//     1 <= present[i], future[i] <= 50
//     hierarchy.length == n - 1
//     hierarchy[i] == [ui, vi]
//     1 <= ui, vi <= n
//     ui != vi
//     1 <= budget <= 160
//     There are no duplicate edges.
//     Employee 1 is the direct or indirect boss of every employee.
//     The input graph hierarchy is guaranteed to have no cycles.
//

struct Solution;

impl Solution {
    pub fn max_profit(n: i32, present: Vec<i32>, future: Vec<i32>, hierarchy: Vec<Vec<i32>>, budget: i32) -> i32 {
        fn dfs(u: i32, present: &[i32], future: &[i32], tree: &[Vec<i32>], dp: &mut Vec<Vec<Vec<i32>>>, budget: i32) {
            let mut child_dps = Vec::new();

            for &v in &tree[u as usize] {
                dfs(v, present, future, tree, dp, budget);
                child_dps.push((dp[v as usize][0].clone(), dp[v as usize][1].clone()));
            }

            let uidx = u as usize;
            for (parent_bought, slot) in dp[uidx].iter_mut().enumerate() {
                let price = if parent_bought == 1 { present[uidx] / 2 } else { present[uidx] };
                let profit = future[uidx] - price;

                let mut curr = vec![0; (budget + 1) as usize];

                // Option 1: don't buy u
                let mut base = vec![0; (budget + 1) as usize];
                for (child_base, _) in &child_dps {
                    let mut next = vec![0; (budget + 1) as usize];
                    for b1 in 0..=budget {
                        if base[b1 as usize] == 0 && b1 != 0 {
                            continue;
                        }
                        for b2 in 0..=budget - b1 {
                            next[(b1 + b2) as usize] = next[(b1 + b2) as usize].max(base[b1 as usize] + child_base[b2 as usize]);
                        }
                    }
                    base = next;
                }

                for b in 0..=budget {
                    curr[b as usize] = curr[b as usize].max(base[b as usize]);
                }

                // Option 2: buy u
                if price <= budget {
                    let mut base_buy = vec![0; (budget + 1) as usize];
                    for (_child_base, child_buy) in &child_dps {
                        let mut next = vec![0; (budget + 1) as usize];
                        for b1 in 0..=budget {
                            if base_buy[b1 as usize] == 0 && b1 != 0 {
                                continue;
                            }
                            for b2 in 0..=budget - b1 {
                                next[(b1 + b2) as usize] = next[(b1 + b2) as usize].max(base_buy[b1 as usize] + child_buy[b2 as usize]);
                            }
                        }
                        base_buy = next;
                    }

                    for b in price..=budget {
                        curr[b as usize] = curr[b as usize].max(base_buy[(b - price) as usize] + profit);
                    }
                }
                *slot = curr;
            }
        }

        let mut tree = vec![Vec::new(); n as usize];
        for edge in &hierarchy {
            tree[(edge[0] - 1) as usize].push(edge[1] - 1);
        }
        let mut dp = vec![vec![vec![0; (budget + 1) as usize]; 2]; n as usize];
        dfs(0, &present, &future, &tree, &mut dp, budget);
        let mut ans = 0;
        for b in 0..=budget {
            ans = ans.max(dp[0][0][b as usize]);
        }
        ans
    }
}

#[test]
fn test() {
    let n = 2;
    let present = vec![1, 2];
    let future = vec![4, 3];
    let hierarchy = vec![vec![1, 2]];
    let budget = 3;
    assert_eq!(Solution::max_profit(n, present, future, hierarchy, budget), 5);

    let n = 2;
    let present = vec![3, 4];
    let future = vec![5, 8];
    let hierarchy = vec![vec![1, 2]];
    let budget = 4;
    assert_eq!(Solution::max_profit(n, present, future, hierarchy, budget), 4);

    let n = 3;
    let present = vec![4, 6, 8];
    let future = vec![7, 9, 11];
    let hierarchy = vec![vec![1, 2], vec![1, 3]];
    let budget = 10;
    assert_eq!(Solution::max_profit(n, present, future, hierarchy, budget), 10);

    let n = 3;
    let present = vec![5, 2, 3];
    let future = vec![8, 5, 6];
    let hierarchy = vec![vec![1, 2], vec![2, 3]];
    let budget = 7;
    assert_eq!(Solution::max_profit(n, present, future, hierarchy, budget), 12);
}
