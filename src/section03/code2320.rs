#![allow(dead_code)]

/*

// 2320. Count Number of Ways to Place Houses
// https://leetcode.com/problems/count-number-of-ways-to-place-houses/
// https://leetcode.cn/problems/count-number-of-ways-to-place-houses/
//
// Medium
//
// There is a street with n * 2 plots, where there are n plots on each side of the street.
// The plots on each side are numbered from 1 to n. On each plot, a house can be placed.

Return the number of ways houses can be placed such that no two houses are adjacent to each other on the same side of the street. Since the answer may be very large, return it modulo 109 + 7.

Note that if a house is placed on the ith plot on one side of the street, a house can also be placed on the ith plot on the other side of the street.

Example 1:

Input: n = 1
Output: 4
Explanation:
Possible arrangements:
1. All plots are empty.
2. A house is placed on one side of the street.
3. A house is placed on the other side of the street.
4. Two houses are placed, one on each side of the street.

Example 2:

Input: n = 2
Output: 9
Explanation: The 9 possible arrangements are shown in the diagram above.

Constraints:

    1 <= n <= 10^4
*/

struct Solution;

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut cache = [1, 4, 9];
        if n <= 2 {
            cache[n as usize]
        } else {
            for _ in 3..=n {
                let val = (2_i32)
                    .wrapping_mul(cache[2])
                    .wrapping_rem_euclid(MOD)
                    .wrapping_add(2_i32.wrapping_mul(cache[1]).wrapping_rem_euclid(MOD))
                    .wrapping_sub(cache[0])
                    .wrapping_rem_euclid(MOD);
                cache[0] = cache[1];
                cache[1] = cache[2];
                cache[2] = val;
            }
            cache[2]
        }
    }
}

#[test]
fn test() {
    let cases = vec![(1, 4), (2, 9), (3, 25), (4, 64), (5, 169), (6, 441), (7, 1156)];
    for (n, expect) in cases {
        assert_eq!(Solution::count_house_placements(n), expect);
    }
}
