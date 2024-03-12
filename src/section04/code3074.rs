#![allow(dead_code)]

// 3074. Apple Redistribution into Boxes
// https://leetcode.com/problems/apple-redistribution-into-boxes/
// https://leetcode.cn/problems/apple-redistribution-into-boxes/
//
// Easy
//
// You are given an array apple of size n and an array capacity of size m.
//
// There are n packs where the ith pack contains apple[i] apples.
// There are m boxes as well, and the ith box has a capacity of capacity[i] apples.
//
// Return the minimum number of boxes you need to select to redistribute these n packs of apples into boxes.
//
// Note that, apples from the same pack can be distributed into different boxes.
//
// Example 1:
//
// Input: apple = [1,3,2], capacity = [4,3,1,5,2]
// Output: 2
// Explanation: We will use boxes with capacities 4 and 5.
// It is possible to distribute the apples as the total capacity is greater than or equal to the total number of apples.
//
// Example 2:
//
// Input: apple = [5,5,5], capacity = [2,4,2,7]
// Output: 4
// Explanation: We will need to use all the boxes.
//
// Constraints:
//
// 1 <= n == apple.length <= 50
// 1 <= m == capacity.length <= 50
// 1 <= apple[i], capacity[i] <= 50
// The input is generated such that it's possible to redistribute packs of apples into boxes.
//

struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        let mut t = 0;
        let mut need: i32 = apple.into_iter().sum();
        capacity.sort();
        for have in capacity.into_iter().rev() {
            if need == 0 {
                break;
            }
            let take = need.min(have);
            need = 0.max(need - take);
            t += 1;
        }
        t
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]), 2);
    assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    assert_eq!(Solution::minimum_boxes(vec![1, 2, 3], vec![1, 2, 3]), 3);
}
