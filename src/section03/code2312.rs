#![allow(dead_code)]

/*

// 2312. Selling Pieces of Wood
// https://leetcode.com/problems/selling-pieces-of-wood/
// https://leetcode.cn/problems/selling-pieces-of-wood/
//
// Hard
//
// You are given two integers m and n that represent the height and width of a rectangular piece of wood.
// You are also given a 2D integer array prices, where prices[i] = [hi, wi, pricei] indicates
// you can sell a rectangular piece of wood of height hi and width wi for pricei dollars.

To cut a piece of wood, you must make a vertical or horizontal cut across the entire height or width of the piece to split it into two smaller pieces. After cutting a piece of wood into some number of smaller pieces, you can sell pieces according to prices. You may sell multiple pieces of the same shape, and you do not have to sell all the shapes. The grain of the wood makes a difference, so you cannot rotate a piece to swap its height and width.

Return the maximum money you can earn after cutting an m x n piece of wood.

Note that you can cut the piece of wood as many times as you want.

Example 1:

Input: m = 3, n = 5, prices = [[1,4,2],[2,2,7],[2,1,3]]
Output: 19
Explanation: The diagram above shows a possible scenario. It consists of:
- 2 pieces of wood shaped 2 x 2, selling for a price of 2 * 7 = 14.
- 1 piece of wood shaped 2 x 1, selling for a price of 1 * 3 = 3.
- 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
This obtains a total of 14 + 3 + 2 = 19 money earned.
It can be shown that 19 is the maximum amount of money that can be earned.

Example 2:

Input: m = 4, n = 6, prices = [[3,2,10],[1,4,2],[4,1,3]]
Output: 32
Explanation: The diagram above shows a possible scenario. It consists of:
- 3 pieces of wood shaped 3 x 2, selling for a price of 3 * 10 = 30.
- 1 piece of wood shaped 1 x 4, selling for a price of 1 * 2 = 2.
This obtains a total of 30 + 2 = 32 money earned.
It can be shown that 32 is the maximum amount of money that can be earned.
Notice that we cannot rotate the 1 x 4 piece of wood to obtain a 4 x 1 piece of wood.

Constraints:

    1 <= m, n <= 200
    1 <= prices.length <= 2 * 10^4
    prices[i].length == 3
    1 <= hi <= m
    1 <= wi <= n
    1 <= pricei <= 10^6
    All the shapes of wood (hi, wi) are pairwise distinct.
*/

struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        fn dfs(high: i32, wid: i32, part_price: &HashMap<(i32, i32), i64>, f: &mut HashMap<(i32, i32), i64>) -> i64 {
            let mut ans: i64 = 0;
            if f.contains_key(&(high, wid)) {
                return f[&(high, wid)];
            }
            if part_price.contains_key(&(high, wid)) {
                ans = part_price[&(high, wid)];
            }
            let half_high = high / 2 + 1;
            let half_wid = wid / 2 + 1;
            for h in 1..half_high {
                let up = dfs(h, wid, part_price, f);
                let down = dfs(high - h, wid, part_price, f);
                ans = ans.max(up + down);
            }
            for w in 1..half_wid {
                let left = dfs(high, w, part_price, f);
                let right = dfs(high, wid - w, part_price, f);
                ans = ans.max(left + right);
            }
            f.insert((high, wid), ans);
            ans
        }

        let part_price: HashMap<(i32, i32), i64> = prices.iter().map(|x| ((x[0], x[1]), x[2] as i64)).collect();
        let mut f: HashMap<(i32, i32), i64> = HashMap::new();
        dfs(m, n, &part_price, &mut f)
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]], 19),
        (4, 6, vec![vec![3, 2, 10], vec![1, 4, 2], vec![4, 1, 3]], 32),
    ];
    for (m, n, prices, expect) in cases {
        assert_eq!(Solution::selling_wood(m, n, prices), expect);
    }
}
