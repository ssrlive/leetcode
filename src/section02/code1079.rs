#![allow(dead_code)]

// 1079. Letter Tile Possibilities
// https://leetcode.com/problems/letter-tile-possibilities/
// https://leetcode.cn/problems/letter-tile-possibilities/
//
// You have n  tiles, where each tile has one letter tiles[i] printed on it.
//
// Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.
//
// Example 1:
//
// Input: tiles = "AAB"
// Output: 8
// Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
//
// Example 2:
//
// Input: tiles = "AAABBC"
// Output: 188
//
// Example 3:
//
// Input: tiles = "V"
// Output: 1
//
// Constraints:
//
// - 1 <= tiles.length <= 7
// - tiles consists of uppercase English letters.
//

struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        use std::collections::*;
        let n = tiles.len();
        let mut map = HashMap::new();
        for c in tiles.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut result = 0;
        let mut set = HashSet::new();
        set.insert(String::new());
        for _ in 0..n {
            let mut new_set = HashSet::new();
            for s in set {
                let mut smap = HashMap::new();
                for c in s.chars() {
                    *smap.entry(c).or_insert(0) += 1;
                }

                for (c, num1) in &map {
                    if let Some(num2) = smap.get(c) {
                        if num2 < num1 {
                            new_set.insert(format!("{s}{c}"));
                        }
                    } else {
                        new_set.insert(format!("{s}{c}"));
                    }
                }
            }
            result += new_set.len() as i32;
            set = new_set;
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_tile_possibilities("AAB".to_string()), 8);
    assert_eq!(Solution::num_tile_possibilities("AAABBC".to_string()), 188);
    assert_eq!(Solution::num_tile_possibilities("V".to_string()), 1);
}
