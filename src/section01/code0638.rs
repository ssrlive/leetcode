#![allow(dead_code)]

// 638. Shopping Offers
// https://leetcode.com/problems/shopping-offers/
// https://leetcode.cn/problems/shopping-offers/
//
// In LeetCode Store, there are n items to sell. Each item has a price. However, there are some special offers,
// and a special offer consists of one or more different kinds of items with a sale price.
//
// You are given an integer array price where price[i] is the price of the ith item, and an integer array needs
// where needs[i] is the number of pieces of the ith item you want to buy.
//
// You are also given an array special where special[i] is of size n + 1 where special[i][j] is the number of pieces of
// the j^th item in the i^th offer and special[i][n] (i.e., the last integer in the array) is the price of the ith offer.
//
// Return the lowest price you have to pay for exactly certain items as given, where you could make optimal use
// of the special offers. You are not allowed to buy more items than you want, even if that would lower the overall price.
// You could use any of the special offers as many times as you want.
//
// Example 1:
//
// Input: price = [2,5], special = [[3,0,5],[1,2,10]], needs = [3,2]
// Output: 14
// Explanation: There are two kinds of items, A and B. Their prices are $2 and $5 respectively.
// In special offer 1, you can pay $5 for 3A and 0B
// In special offer 2, you can pay $10 for 1A and 2B.
// You need to buy 3A and 2B, so you may pay $10 for 1A and 2B (special offer #2), and $4 for 2A.
//
// Example 2:
//
// Input: price = [2,3,4], special = [[1,1,0,4],[2,2,1,9]], needs = [1,2,1]
// Output: 11
// Explanation: The price of A is $2, and $3 for B, $4 for C.
// You may pay $4 for 1A and 1B, and $9 for 2A ,2B and 1C.
// You need to buy 1A ,2B and 1C, so you may pay $4 for 1A and 1B (special offer #1), and $3 for 1B, $4 for 1C.
// You cannot add more items, though only $9 for 2A ,2B and 1C.
//
// Constraints:
//
// - n == price.length == needs.length
// - 1 <= n <= 6
// - 0 <= price[i], needs[i] <= 10
// - 1 <= special.length <= 100
// - special[i].length == n + 1
// - 0 <= special[i][j] <= 50
//

struct Solution;

impl Solution {
    fn is_valid(needs: &[i32], special: &[Vec<i32>], i: usize) -> bool {
        for (j, &item) in needs.iter().enumerate() {
            if special[i][j] > item {
                return false;
            }
        }
        true
    }
    fn is_empty(needs: &Vec<i32>) -> bool {
        for item in needs {
            if *item != 0 {
                return false;
            }
        }
        true
    }
    fn solve(
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        needs: &Vec<i32>,
        dp: &mut std::collections::HashMap<String, i32>,
    ) -> i32 {
        let k = needs.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
        if let Some(&v) = dp.get(&k) {
            return v;
        }
        if Self::is_empty(needs) {
            dp.insert(k, 0);
            return 0;
        }
        let mut min_price = std::i32::MAX;
        for i in 0..special.len() {
            if Self::is_valid(needs, special, i) {
                let mut needs = needs.clone();
                for (j, item) in needs.iter_mut().enumerate() {
                    *item -= special[i][j];
                }
                let lprice = special[i][needs.len()] + Self::solve(price, special, &needs, dp);
                min_price = min_price.min(lprice);
            }
        }
        let mut lprice = 0;
        for i in 0..needs.len() {
            lprice += needs[i] * price[i];
        }
        min_price = min_price.min(lprice);

        dp.insert(k, min_price);

        min_price
    }
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut dp = std::collections::HashMap::new();
        Self::solve(&price, &special, &needs, &mut dp)
    }
}

#[test]
fn test() {
    let price = vec![2, 5];
    let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
    let needs = vec![3, 2];
    assert_eq!(Solution::shopping_offers(price, special, needs), 14);

    let price = vec![2, 3, 4];
    let special = vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]];
    let needs = vec![1, 2, 1];
    assert_eq!(Solution::shopping_offers(price, special, needs), 11);

    let price = vec![2, 2];
    let special = vec![
        vec![1, 1, 1],
        vec![1, 1, 2],
        vec![1, 1, 3],
        vec![1, 1, 4],
        vec![1, 1, 5],
        vec![1, 1, 6],
        vec![1, 1, 7],
        vec![1, 1, 8],
        vec![1, 1, 9],
        vec![1, 1, 10],
        vec![1, 1, 11],
        vec![1, 1, 12],
        vec![1, 1, 13],
        vec![1, 1, 14],
        vec![1, 1, 15],
    ];
    let needs = vec![10, 10];
    assert_eq!(Solution::shopping_offers(price, special, needs), 10);
}
