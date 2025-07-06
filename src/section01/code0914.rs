#![allow(dead_code)]

// 914. X of a Kind in a Deck of Cards
// https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
// https://leetcode.cn/problems/x-of-a-kind-in-a-deck-of-cards/
//
// You are given an integer array deck where deck[i] represents the number written on the ith card.
//
// Partition the cards into one or more groups such that:
//
// Each group has exactly x cards where x > 1, and
// All the cards in one group have the same integer written on them.
// Return true if such partition is possible, or false otherwise.
//
// Example 1:
//
// Input: deck = [1,2,3,4,4,3,2,1]
// Output: true
// Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].
//
// Example 2:
//
// Input: deck = [1,1,1,2,2,2,3,3]
// Output: false
// Explanation: No possible partition.
//
// Constraints:
//
// - 1 <= deck.length <= 10^4
// - 0 <= deck[i] < 10^4
//

struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        use std::collections::HashMap;

        fn find_gcd(mut x: u32, mut y: u32) -> u32 {
            let mut rem = x % y;
            while rem != 0 {
                x = y;
                y = rem;
                rem = x % y;
            }
            y
        }

        if deck.len() < 2 {
            return false;
        }
        let mut count_map: HashMap<i32, u32> = HashMap::new();
        for n in deck {
            let c = count_map.entry(n).or_insert(0);
            *c += 1;
        }
        if count_map.len() < 2 {
            return true;
        }
        let mut counts = count_map.values().copied().collect::<Vec<u32>>();
        counts.sort_unstable();
        counts.dedup();
        if counts.len() < 2 {
            return true;
        }
        let gcd = find_gcd(counts[1], counts[0]);
        if gcd < 2 {
            return false;
        }
        count_map.values().all(|&x| x.is_multiple_of(gcd))
    }
}

#[test]
fn test() {
    assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
    assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    assert!(!Solution::has_groups_size_x(vec![1]));
    assert!(Solution::has_groups_size_x(vec![1, 1]));
}
