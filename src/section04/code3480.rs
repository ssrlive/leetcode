#![allow(dead_code)]

// 3480. Maximize Subarrays After Removing One Conflicting Pair
// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/
// https://leetcode.cn/problems/maximize-subarrays-after-removing-one-conflicting-pair/
//
// Hard
//
// You are given an integer n which represents an array nums containing the numbers from 1 to n in order. Additionally, you are given a 2D array
// conflictingPairs, where conflictingPairs[i] = [a, b] indicates that a and b form a conflicting pair.
//
// Remove exactly one element from conflictingPairs. Afterward, count the number of non-empty subarrays of nums which
// do not contain both a and b for any remaining conflicting pair [a, b].
//
// Return the maximum number of subarrays possible after removing exactly one conflicting pair.
//
// Example 1:
//
// Input: n = 4, conflictingPairs = [[2,3],[1,4]]
//
// Output: 9
//
// Explanation:
//
// Remove [2, 3] from conflictingPairs. Now, conflictingPairs = [[1, 4]].
// There are 9 subarrays in nums where [1, 4] do not appear together.
// They are [1], [2], [3], [4], [1, 2], [2, 3], [3, 4], [1, 2, 3] and [2, 3, 4].
// The maximum number of subarrays we can achieve after removing one element from conflictingPairs is 9.
//
// Example 2:
//
// Input: n = 5, conflictingPairs = [[1,2],[2,5],[3,5]]
//
// Output: 12
//
// Explanation:
//
// Remove [1, 2] from conflictingPairs. Now, conflictingPairs = [[2, 5], [3, 5]].
// There are 12 subarrays in nums where [2, 5] and [3, 5] do not appear together.
// The maximum number of subarrays we can achieve after removing one element from conflictingPairs is 12.
//
// Constraints:
//
// 2 <= n <= 10^5
// 1 <= conflictingPairs.length <= 2 * n
// conflictingPairs[i].length == 2
// 1 <= conflictingPairs[i][j] <= n
// conflictingPairs[i][0] != conflictingPairs[i][1]
//

struct Solution;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        let m = conflicting_pairs.len() as i64;

        let mut add_at = vec![vec![]; (n + 1) as usize];
        for k in 0..m {
            let a = conflicting_pairs[k as usize][0] as i64;
            let b = conflicting_pairs[k as usize][1] as i64;
            let l = std::cmp::min(a, b);
            let r = std::cmp::max(a, b);
            add_at[l as usize].push((r, k));
        }

        let mut active = std::collections::BTreeSet::new();
        let mut cnt = vec![0; (n + 1) as usize];
        let mut base_l_line = 0;
        let mut improvement = vec![0; m as usize];

        for i in (1..=n).rev() {
            for &(r, k) in &add_at[i as usize] {
                active.insert((r, k));
                cnt[r as usize] += 1;
            }

            if active.is_empty() {
                base_l_line += n + 1 - i;
            } else {
                let minimum_r = active.iter().next().unwrap().0;
                base_l_line += minimum_r - i;

                if cnt[minimum_r as usize] == 1 {
                    let k_i = active.iter().next().unwrap().1;
                    let next_it = active.iter().position(|x| *x > (minimum_r, m));
                    let second_minimum_r = next_it.map(|x| active.iter().nth(x).unwrap().0).unwrap_or(n + 1);
                    improvement[k_i as usize] += second_minimum_r - minimum_r;
                }
            }
        }

        let max_improvement = improvement.iter().max().unwrap();
        base_l_line + max_improvement
    }
}

#[test]
fn test() {
    let n = 4;
    let conflicting_pairs = vec![vec![2, 3], vec![1, 4]];
    let output = 9;
    assert_eq!(Solution::max_subarrays(n, conflicting_pairs), output);

    let n = 5;
    let conflicting_pairs = vec![vec![1, 2], vec![2, 5], vec![3, 5]];
    let output = 12;
    assert_eq!(Solution::max_subarrays(n, conflicting_pairs), output);
}
