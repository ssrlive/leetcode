#![allow(dead_code)]

// 2561. Rearranging Fruits
// https://leetcode.com/problems/rearranging-fruits/
// https://leetcode.cn/problems/rearranging-fruits/
//
// Hard
//
// You have two fruit baskets containing n fruits each. You are given two 0-indexed integer arrays basket1 and basket2 representing the cost of fruit in each basket. You want to make both baskets equal. To do so, you can use the following operation as many times as you want:
//
//     Chose two indices i and j, and swap the ith fruit of basket1 with the jth fruit of basket2.
//     The cost of the swap is min(basket1[i],basket2[j]).
//
// Two baskets are considered equal if sorting them according to the fruit cost makes them exactly the same baskets.
//
// Return the minimum cost to make both the baskets equal or -1 if impossible.
//
// Example 1:
//
// Input: basket1 = [4,2,2,2], basket2 = [1,4,1,2]
// Output: 1
// Explanation: Swap index 1 of basket1 with index 0 of basket2, which has cost 1. Now basket1 = [4,1,2,2] and basket2 = [2,4,1,2]. Rearranging both the arrays makes them equal.
//
// Example 2:
//
// Input: basket1 = [2,3,4,1], basket2 = [3,2,5,1]
// Output: -1
// Explanation: It can be shown that it is impossible to make both the baskets equal.
//
// Constraints:
//
// -    basket1.length == bakste2.length
// -    1 <= basket1.length <= 10^5
// -    1 <= basket1[i],basket2[i] <= 10^9
//

struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut m = std::collections::BTreeMap::<i64, i64>::new();
        for c in basket1 {
            *m.entry(c as i64).or_insert(0) += 1;
        }
        for c in basket2 {
            *m.entry(c as i64).or_insert(0) -= 1;
        }
        let mut swaps = 0;
        let mut res = 0;
        for &cnt in m.values() {
            if cnt % 2 != 0 {
                return -1;
            }
            swaps += (cnt / 2).max(0);
        }
        let first = *m.iter().next().unwrap().0;
        for (&c, &cnt) in &m {
            let take = swaps.min(cnt.abs() / 2);
            res += take * c.min(first * 2);
            swaps -= take;
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![(vec![4, 2, 2, 2], vec![1, 4, 1, 2], 1), (vec![2, 3, 4, 1], vec![3, 2, 5, 1], -1)];
    for (basket1, basket2, expected) in cases {
        assert_eq!(Solution::min_cost(basket1, basket2), expected);
    }
}
