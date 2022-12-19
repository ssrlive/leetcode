#![allow(dead_code)]

// 135. Candy
// https://leetcode.com/problems/candy/
// https://leetcode.cn/problems/candy/
//
// There are N children standing in a line. Each child is assigned a rating value.
//
// You are giving candies to these children subjected to the following requirements:
// - Each child must have at least one candy.
// - Children with a higher rating get more candies than their neighbors.
//
// Return the minimum number of candies you need to have to distribute the candies to the children.
//

struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];

        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }

        for i in (0..ratings.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }

        candies.iter().sum()
    }
}

#[test]
fn test_candy() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
}
