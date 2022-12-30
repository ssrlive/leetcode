#![allow(dead_code)]

// 888. Fair Candy Swap
// https://leetcode.com/problems/fair-candy-swap/
// https://leetcode.cn/problems/fair-candy-swap/
//
// Alice and Bob have a different total number of candies. You are given two integer arrays aliceSizes and bobSizes where aliceSizes[i] is the
// number of candies of the ith box of candy that Alice has and bobSizes[j] is the number of candies of the jth box of candy that Bob has.
//
// Since they are friends, they would like to exchange one candy box each so that after the exchange, they both have the same total amount
// of candy. The total amount of candy a person has is the sum of the number of candies in each box they have.
//
// Return an integer array answer where answer[0] is the number of candies in the box that Alice must exchange,
// and answer[1] is the number of candies in the box that Bob must exchange. If there are multiple answers,
// you may return any one of them. It is guaranteed that at least one answer exists.
//
// Example 1:
//
// Input: aliceSizes = [1,1], bobSizes = [2,2]
// Output: [1,2]
//
// Example 2:
//
// Input: aliceSizes = [1,2], bobSizes = [2,3]
// Output: [1,2]
//
// Example 3:
//
// Input: aliceSizes = [2], bobSizes = [1,3]
// Output: [2,3]
//
// Constraints:
//
// - 1 <= aliceSizes.length, bobSizes.length <= 10^4
// - 1 <= aliceSizes[i], bobSizes[j] <= 10^5
// - Alice and Bob have a different total number of candies.
// - There will be at least one valid answer for the given input.
//

struct Solution;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let a_sum: i32 = alice_sizes.iter().sum();
        let b_sum: i32 = bob_sizes.iter().sum();
        let a_set: HashSet<i32> = alice_sizes.into_iter().collect();
        let mut trade: Vec<i32> = vec![];

        for b in bob_sizes.iter() {
            let a = (a_sum - b_sum) / 2 + b;
            if a_set.contains(&a) {
                trade.push(a);
                trade.push(*b);
                if trade.len() == 2 {
                    break;
                }
            }
        }
        trade
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1], vec![2, 2], vec![1, 2]),
        (vec![1, 2], vec![2, 3], vec![1, 2]),
        (vec![2], vec![1, 3], vec![2, 3]),
    ];
    for (alice_sizes, bob_sizes, expected) in cases {
        assert_eq!(Solution::fair_candy_swap(alice_sizes, bob_sizes), expected);
    }
}
