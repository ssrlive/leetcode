#![allow(dead_code)]

// 904. Fruit Into Baskets
// https://leetcode.com/problems/fruit-into-baskets/
// https://leetcode.cn/problems/fruit-into-baskets/
//
// You are visiting a farm that has a single row of fruit trees arranged from left to right.
// The trees are represented by an integer array fruits where fruits[i] is the type of fruit the ith tree produces.
//
// You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:
//
// You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
// Starting from any tree of your choice, you must pick exactly one fruit from every tree
// (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
// Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
// Given the integer array fruits, return the maximum number of fruits you can pick.
//
// Example 1:
//
// Input: fruits = [1,2,1]
// Output: 3
// Explanation: We can pick from all 3 trees.
//
// Example 2:
//
// Input: fruits = [0,1,2,2]
// Output: 3
// Explanation: We can pick from trees [1,2,2].
// If we had started at the first tree, we would only pick from trees [0,1].
//
// Example 3:
//
// Input: fruits = [1,2,3,2,2]
// Output: 4
// Explanation: We can pick from trees [2,3,2,2].
// If we had started at the first tree, we would only pick from trees [1,2].
//
// Constraints:
//
// - 1 <= fruits.length <= 10^5
// - 0 <= fruits[i] < fruits.length
//

struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let mut start_first: usize = 0;
        let mut start_second: usize = 0;
        let mut end_first: usize = 0;
        let mut end_second: usize = 0;
        let mut first_bag: Option<i32> = None;
        let mut second_bag: Option<i32> = None;
        let mut max_q: usize = 0;

        for (i, &fruit) in fruits.iter().enumerate() {
            if first_bag.is_none() {
                first_bag = Some(fruit);
                start_first = i;
                end_first = i + 1;
            } else if first_bag.unwrap() == fruit {
                end_first = i + 1;
            } else if second_bag.is_none() {
                second_bag = Some(fruit);
                start_second = i;
                end_second = i + 1;
            } else if second_bag.unwrap() == fruit {
                end_second = i + 1;
            } else {
                max_q = max(max_q, max(end_second, end_first) - min(start_first, start_second));
                if end_second > end_first {
                    start_second = end_first;

                    end_first = i + 1;
                    first_bag = Some(fruit);
                    start_first = i;
                } else {
                    start_first = end_second;

                    second_bag = Some(fruit);
                    start_second = i;
                    end_second = i + 1;
                }
            }
        }
        max(max_q, max(end_second, end_first) - min(start_first, start_second)) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    assert_eq!(Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
}
