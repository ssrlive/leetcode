#![allow(dead_code)]

/*

// 1718. Construct the Lexicographically Largest Valid Sequence
// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
// https://leetcode.cn/problems/construct-the-lexicographically-largest-valid-sequence/
//
// Medium
//
// Given an integer n, find a sequence that satisfies all of the following:

    The integer 1 occurs once in the sequence.
    Each integer between 2 and n occurs twice in the sequence.
    For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.

The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their indices, |j - i|.

Return the lexicographically largest sequence. It is guaranteed that under the given constraints, there is always a solution.

A sequence a is lexicographically larger than a sequence b (of the same length) if in the first position where a and b differ, sequence a has a number greater than the corresponding number in b. For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they differ is at the third number, and 9 is greater than 5.

Example 1:

Input: n = 3
Output: [3,1,2,3,2]
Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.

Example 2:

Input: n = 5
Output: [5,3,1,4,3,5,2,4,2]

Constraints:

    1 <= n <= 20
*/

struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn dfs(pos: usize, ans: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
            if pos == ans.len() {
                return true;
            }
            if ans[pos] != 0 {
                return dfs(pos + 1, ans, used);
            }
            for i in (1..used.len()).rev() {
                if used[i] {
                    continue;
                }
                if i != 1 && (pos + i >= ans.len() || ans[pos + i] != 0) {
                    continue;
                }
                ans[pos] = i as i32;
                if i != 1 {
                    ans[pos + i] = i as i32;
                }
                used[i] = true;
                if dfs(pos + 1, ans, used) {
                    return true;
                }
                ans[pos] = 0;
                if i != 1 {
                    ans[pos + i] = 0;
                }
                used[i] = false;
            }
            false
        }

        let n = n as usize;
        let mut ans = vec![0; 2 * n - 1];
        let mut used = vec![false; n + 1];
        dfs(0, &mut ans, &mut used);
        ans
    }
}

#[test]
fn test() {
    let n = 3;
    let res = vec![3, 1, 2, 3, 2];
    assert_eq!(Solution::construct_distanced_sequence(n), res);
    let n = 5;
    let res = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];
    assert_eq!(Solution::construct_distanced_sequence(n), res);
    let n = 20;
    let res = vec![
        20, 18, 19, 15, 13, 17, 10, 16, 7, 5, 3, 14, 12, 3, 5, 7, 10, 13, 15, 18, 20, 19, 17, 16, 12, 14, 11, 9, 4, 6,
        8, 2, 4, 2, 1, 6, 9, 11, 8,
    ];
    assert_eq!(Solution::construct_distanced_sequence(n), res);
}
