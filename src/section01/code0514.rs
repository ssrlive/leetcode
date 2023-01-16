#![allow(dead_code)]

// 514. Freedom Trail
// https://leetcode.com/problems/freedom-trail/
// https://leetcode.cn/problems/freedom-trail/
//
// In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called
// the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.
//
// Given a string ring that represents the code engraved on the outer ring and another string key that represents
// the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.
//
// Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters
// in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned
// at the "12:00" direction and then by pressing the center button.
//
// At the stage of rotating the ring to spell the key character key[i]:
//
// You can rotate the ring clockwise or anticlockwise by one place, which counts as one step.
// The final purpose of the rotation is to align one of ring's characters at the "12:00" direction,
// where this character must equal key[i].
// If the character key[i] has been aligned at the "12:00" direction, press the center button to spell,
// which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage).
// Otherwise, you have finished all the spelling.
//
// Example 1:
//
// Input: ring = "godding", key = "gd"
// Output: 4
// Explanation:
// For the first key character 'g', since it is already in place, we just need 1 step to spell this character.
// For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
// Also, we need 1 more step for spelling.
// So the final output is 4.
//
// Example 2:
//
// Input: ring = "godding", key = "godding"
// Output: 13
//
// Constraints:
//
// - 1 <= ring.length, key.length <= 100
// - ring and key consist of only lower case English letters.
// - It is guaranteed that key could always be spelled by rotating ring.
//

struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut f: Vec<Vec<i64>> = vec![vec![std::i32::MAX as i64; ring.len()]; key.len()];
        let mut map = std::collections::HashMap::<char, Vec<usize>>::new();
        for (i, c) in ring.char_indices() {
            map.entry(c).or_default().push(i);
        }
        let key: Vec<char> = key.chars().collect();
        for &i in map[&key[0]].iter() {
            f[0][i] = (ring.len() as i64 - i as i64).abs().min(i as i64);
        }

        for k in 1..key.len() {
            for &to in map[&key[k]].iter() {
                for &from in map[&key[k - 1]].iter() {
                    let diff = (from as i64 - to as i64).abs();
                    let diff = diff.min((ring.len() as i64 - diff).abs());
                    f[k][to] = f[k][to].min(f[k - 1][from] + diff);
                }
            }
        }
        map[&key[key.len() - 1]]
            .iter()
            .fold(std::i64::MAX, |ret, idx| ret.min(f[key.len() - 1][*idx])) as i32
            + key.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_rotate_steps("godding".to_string(), "gd".to_string()), 4);
    assert_eq!(
        Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
        13
    );
}
