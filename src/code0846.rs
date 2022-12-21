#![allow(dead_code)]

// 846. Hand of Straights
// https://leetcode.com/problems/hand-of-straights/
// https://leetcode.cn/problems/hand-of-straights/
//
// Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.
//
// Given an integer array hand where hand[i] is the value written on the ith card and an integer groupSize, return true if she can rearrange the cards, or false otherwise.
//
// Example 1:
//
// Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
// Output: true
// Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]
//
// Example 2:
//
// Input: hand = [1,2,3,4,5], groupSize = 4
// Output: false
// Explanation: Alice's hand can not be rearranged into groups of 4.
//
// Constraints:
//
// - 1 <= hand.length <= 10^4
// - 0 <= hand[i] <= 10^9
// - 1 <= groupSize <= hand.length
//
// Note: This question is the same as 1296: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
//

struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let n = hand.len();
        let mut index = 0;
        let mut map = std::collections::HashMap::new();

        if n % group_size as usize != 0 {
            return false;
        }

        let mut hand = hand;
        hand.sort();
        for x in hand.iter() {
            *map.entry(x).or_insert(0) += 1;
        }

        while index < n && map.len() >= group_size as usize {
            if !map.contains_key(&hand[index]) {
                index += 1;
                continue;
            }
            let mut start = hand[index];
            while (start - hand[index]) < group_size {
                if !map.contains_key(&start) {
                    return false;
                }
                *map.get_mut(&start).unwrap() -= 1;
                if *map.get(&start).unwrap() == 0 {
                    map.remove(&start);
                }
                start += 1;
            }
        }
        map.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
    assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
}
