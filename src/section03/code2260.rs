#![allow(dead_code)]

/*

// 2260. Minimum Consecutive Cards to Pick Up
// https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/
// https://leetcode.cn/problems/minimum-consecutive-cards-to-pick-up/
//
// Medium
//
// You are given an integer array cards where cards[i] represents the value of the ith card. A pair of cards are matching if the cards have the same value.

Return the minimum number of consecutive cards you have to pick up to have a pair of matching cards among the picked cards. If it is impossible to have matching cards, return -1.

Example 1:

Input: cards = [3,4,2,3,4,7]
Output: 4
Explanation: We can pick up the cards [3,4,2,3] which contain a matching pair of cards with value 3. Note that picking up the cards [4,2,3,4] is also optimal.

Example 2:

Input: cards = [1,0,5,3]
Output: -1
Explanation: There is no way to pick up a set of consecutive cards that contain a pair of matching cards.

Constraints:

    1 <= cards.length <= 10^5
    0 <= cards[i] <= 10^6
*/

struct Solution;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        use std::cmp::min;
        use std::collections::HashMap;
        let mut ans = i32::MAX;
        let mut vi = HashMap::new();
        for (i, v) in cards.iter().enumerate() {
            if !vi.contains_key(v) {
                vi.insert(v, i);
                continue;
            }
            let pre = vi[v];
            let curr = i - pre + 1;
            ans = min(ans, curr as i32);
            vi.insert(v, i);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
    assert_eq!(Solution::minimum_card_pickup(vec![1, 0, 5, 3]), -1);
}
