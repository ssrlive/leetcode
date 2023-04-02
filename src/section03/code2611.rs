#![allow(dead_code)]

/*

// 2611. Mice and Cheese
// https://leetcode.com/problems/mice-and-cheese/
// https://leetcode.cn/problems/mice-and-cheese/
//
// Medium
//
// There are two mice and n different types of cheese, each type of cheese should be eaten by exactly one mouse.

A point of the cheese with index i (0-indexed) is:

    reward1[i] if the first mouse eats it.
    reward2[i] if the second mouse eats it.

You are given a positive integer array reward1, a positive integer array reward2, and a non-negative integer k.

Return the maximum points the mice can achieve if the first mouse eats exactly k types of cheese.

Example 1:

Input: reward1 = [1,1,3,4], reward2 = [4,4,1,1], k = 2
Output: 15
Explanation: In this example, the first mouse eats the 2nd (0-indexed) and the 3rd types of cheese, and the second mouse eats the 0th and the 1st types of cheese.
The total points are 4 + 4 + 3 + 4 = 15.
It can be proven that 15 is the maximum total points that the mice can achieve.

Example 2:

Input: reward1 = [1,1], reward2 = [1,1], k = 2
Output: 2
Explanation: In this example, the first mouse eats the 0th (0-indexed) and 1st types of cheese, and the second mouse does not eat any cheese.
The total points are 1 + 1 = 2.
It can be proven that 2 is the maximum total points that the mice can achieve.

Constraints:

    1 <= n == reward1.length == reward2.length <= 10^5
    1 <= reward1[i], reward2[i] <= 1000
    0 <= k <= n
*/

struct Solution;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut pq = std::collections::BinaryHeap::new();
        let mut ans = 0;
        for i in 0..reward1.len() {
            pq.push(reward1[i] - reward2[i]);
            ans += reward2[i];
        }
        let mut k = k;
        while k != 0 {
            ans += pq.pop().unwrap();
            k -= 1;
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2, 15),
        (vec![1, 1], vec![1, 1], 2, 2),
        (vec![1, 1, 3, 4], vec![4, 4, 1, 1], 1, 13),
    ];
    for (reward1, reward2, k, expected) in cases {
        assert_eq!(Solution::mice_and_cheese(reward1, reward2, k), expected);
    }
}
