#![allow(dead_code)]

/*

// 2300. Successful Pairs of Spells and Potions
// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// https://leetcode.cn/problems/successful-pairs-of-spells-and-potions/
//
// Medium
//
// You are given two positive integer arrays spells and potions, of length n and m respectively, where spells[i] represents the strength of the ith spell and potions[j] represents the strength of the jth potion.

You are also given an integer success. A spell and potion pair is considered successful if the product of their strengths is at least success.

Return an integer array pairs of length n where pairs[i] is the number of potions that will form a successful pair with the ith spell.

Example 1:

Input: spells = [5,1,3], potions = [1,2,3,4,5], success = 7
Output: [4,0,3]
Explanation:
- 0th spell: 5 * [1,2,3,4,5] = [5,10,15,20,25]. 4 pairs are successful.
- 1st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
- 2nd spell: 3 * [1,2,3,4,5] = [3,6,9,12,15]. 3 pairs are successful.
Thus, [4,0,3] is returned.

Example 2:

Input: spells = [3,1,2], potions = [8,5,8], success = 16
Output: [2,0,2]
Explanation:
- 0th spell: 3 * [8,5,8] = [24,15,24]. 2 pairs are successful.
- 1st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
- 2nd spell: 2 * [8,5,8] = [16,10,16]. 2 pairs are successful.
Thus, [2,0,2] is returned.

Constraints:

    n == spells.length
    m == potions.length
    1 <= n, m <= 10^5
    1 <= spells[i], potions[i] <= 10^5
    1 <= success <= 10^10
*/

struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut potions = potions;
        let num_potions = potions.len() as i32;
        potions.sort_unstable();
        let mut result = vec![0; spells.len()];
        for (i, spell) in spells.into_iter().enumerate() {
            let left = potions
                .binary_search_by(|&potion| match (potion as i64 * spell as i64).cmp(&success) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => Ordering::Greater,
                    Ordering::Greater => Ordering::Greater,
                })
                .unwrap_err() as i32;
            result[i] = num_potions - left;
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7, vec![4, 0, 3]),
        (vec![3, 1, 2], vec![8, 5, 8], 16, vec![2, 0, 2]),
    ];
    for (spells, potions, success, expected) in cases {
        assert_eq!(Solution::successful_pairs(spells, potions, success), expected);
    }
}
