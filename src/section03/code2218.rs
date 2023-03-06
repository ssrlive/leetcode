#![allow(dead_code)]

/*

// 2218. Maximum Value of K Coins From Piles
// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/
// https://leetcode.cn/problems/maximum-value-of-k-coins-from-piles/
//
// Hard
//
// There are n piles of coins on a table. Each pile consists of a positive number of coins of assorted denominations.

In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet.

Given a list piles, where piles[i] is a list of integers denoting the composition of the ith pile from top to bottom, and a positive integer k, return the maximum total value of coins you can have in your wallet if you choose exactly k coins optimally.

Example 1:

Input: piles = [[1,100,3],[7,8,9]], k = 2
Output: 101
Explanation:
The above diagram shows the different ways we can choose k coins.
The maximum total we can obtain is 101.

Example 2:

Input: piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
Output: 706
Explanation:
The maximum total can be obtained if we choose all coins from the last pile.

Constraints:

    n == piles.length
    1 <= n <= 1000
    1 <= piles[i][j] <= 10^5
    1 <= k <= sum(piles[i].length) <= 2000
*/

struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        fn prefix(p: Vec<i32>) -> Vec<i32> {
            let mut ret = Vec::new();
            let mut sum = 0;
            for i in p {
                sum += i;
                ret.push(sum);
            }
            ret
        }
        let mut vals = vec![0; k as usize + 1];
        for pile in piles {
            let p = prefix(pile);
            let old = vals.clone();
            for (l, val) in vals.iter_mut().enumerate().take(k as usize + 1).skip(1) {
                let mut current_max = *old.get(l).unwrap();
                for i in 1..=(l.min(p.len())) {
                    let v = old.get(l - i).unwrap() + p.get(i - 1).unwrap();
                    current_max = v.max(current_max);
                }
                *val = current_max;
            }
        }
        *vals.iter().max().unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1, 100, 3], vec![7, 8, 9]], 2, 101),
        (
            vec![
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![1, 1, 1, 1, 1, 1, 700],
            ],
            7,
            706,
        ),
    ];
    for (piles, k, expected) in cases {
        assert_eq!(Solution::max_value_of_coins(piles, k), expected);
    }
}
