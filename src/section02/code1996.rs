#![allow(dead_code)]

/*

// 1996. The Number of Weak Characters in the Game
// https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/
// https://leetcode.cn/problems/the-number-of-weak-characters-in-the-game/
//
// Medium
//
// You are playing a game that contains multiple characters, and each of the characters has two main properties: attack and defense. You are given a 2D integer array properties where properties[i] = [attacki, defensei] represents the properties of the ith character in the game.

A character is said to be weak if any other character has both attack and defense levels strictly greater than this character's attack and defense levels. More formally, a character i is said to be weak if there exists another character j where attackj > attacki and defensej > defensei.

Return the number of weak characters.

Example 1:

Input: properties = [[5,5],[6,3],[3,6]]
Output: 0
Explanation: No character has strictly greater attack and defense than the other.

Example 2:

Input: properties = [[2,2],[3,3]]
Output: 1
Explanation: The first character is weak because the second character has a strictly greater attack and defense.

Example 3:

Input: properties = [[1,5],[10,4],[4,3]]
Output: 1
Explanation: The third character is weak because the second character has a strictly greater attack and defense.

Constraints:

    2 <= properties.length <= 10^5
    properties[i].length == 2
    1 <= attacki, defensei <= 10^5
*/

struct Solution;

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by_key(|pair| (-pair[0], pair[1]));
        properties
            .iter()
            .fold((0, 0), |(res, max_def), pair| {
                (res + (pair[1] < max_def) as i32, (max_def.max(pair[1])))
            })
            .0
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![5, 5], vec![6, 3], vec![3, 6]], 0),
        (vec![vec![2, 2], vec![3, 3]], 1),
        (vec![vec![1, 5], vec![10, 4], vec![4, 3]], 1),
    ];
    for (properties, expected) in cases {
        assert_eq!(Solution::number_of_weak_characters(properties), expected);
    }
}
