#![allow(dead_code)]

// 886. Possible Bipartition
// https://leetcode.com/problems/possible-bipartition/
// https://leetcode.cn/problems/possible-bipartition/
//
// We want to split a group of n people (labeled from 1 to n) into two groups of any size.
// Each person may dislike some other people, and they should not go into the same group.
//
// Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that the person labeled ai
// does not like the person labeled bi, return true if it is possible to split everyone into two groups in this way.
//
// Example 1:
//
// Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
// Output: true
// Explanation: group1 [1,4] and group2 [2,3].
//
// Example 2:
//
// Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
// Output: false
//
// Example 3:
//
// Input: n = 5, dislikes = [[1,2],[2,3],[3,4],[4,5],[1,5]]
// Output: false
//
// Constraints:
//
// - 1 <= n <= 2000
// - 0 <= dislikes.length <= 10^4
// - dislikes[i].length == 2
// - 1 <= dislikes[i][j] <= n
// - ai < bi
// - All the pairs of dislikes are unique.
//

struct Solution;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;

        let mut colors: Vec<i32> = vec![0; n + 1];
        let mut dislike_index: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

        for dislike in dislikes {
            dislike_index[dislike[0] as usize].push(dislike[1] as usize);
            dislike_index[dislike[1] as usize].push(dislike[0] as usize);
        }

        for i in 1..=n {
            if colors[i] == 0 {
                colors[i] = 1;
                let mut stack = vec![i];

                while let Some(idx) = stack.pop() {
                    let next_col = if colors[idx] == 1 { 2 } else { 1 };

                    for &d in dislike_index[idx].iter() {
                        if colors[d] == 0 {
                            colors[d] = next_col;
                            stack.push(d);
                        } else if colors[d] != next_col {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![vec![1, 2], vec![1, 3], vec![2, 4]], true),
        (3, vec![vec![1, 2], vec![1, 3], vec![2, 3]], false),
        (5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]], false),
    ];
    for (n, dislikes, expected) in cases {
        assert_eq!(Solution::possible_bipartition(n, dislikes), expected);
    }
}
