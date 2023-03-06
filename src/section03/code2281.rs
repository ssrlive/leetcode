#![allow(dead_code)]

/*

// 2281. Sum of Total Strength of Wizards
// https://leetcode.com/problems/sum-of-total-strength-of-wizards/
// https://leetcode.cn/problems/sum-of-total-strength-of-wizards/
//
// Hard
//
// As the ruler of a kingdom, you have an army of wizards at your command.

You are given a 0-indexed integer array strength, where strength[i] denotes the strength of the ith wizard. For a contiguous group of wizards (i.e. the wizards' strengths form a subarray of strength), the total strength is defined as the product of the following two values:

    The strength of the weakest wizard in the group.
    The total of all the individual strengths of the wizards in the group.

Return the sum of the total strengths of all contiguous groups of wizards. Since the answer may be very large, return it modulo 109 + 7.

A subarray is a contiguous non-empty sequence of elements within an array.

Example 1:

Input: strength = [1,3,1,2]
Output: 44
Explanation: The following are all the contiguous groups of wizards:
- [1] from [1,3,1,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [3] from [1,3,1,2] has a total strength of min([3]) * sum([3]) = 3 * 3 = 9
- [1] from [1,3,1,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [2] from [1,3,1,2] has a total strength of min([2]) * sum([2]) = 2 * 2 = 4
- [1,3] from [1,3,1,2] has a total strength of min([1,3]) * sum([1,3]) = 1 * 4 = 4
- [3,1] from [1,3,1,2] has a total strength of min([3,1]) * sum([3,1]) = 1 * 4 = 4
- [1,2] from [1,3,1,2] has a total strength of min([1,2]) * sum([1,2]) = 1 * 3 = 3
- [1,3,1] from [1,3,1,2] has a total strength of min([1,3,1]) * sum([1,3,1]) = 1 * 5 = 5
- [3,1,2] from [1,3,1,2] has a total strength of min([3,1,2]) * sum([3,1,2]) = 1 * 6 = 6
- [1,3,1,2] from [1,3,1,2] has a total strength of min([1,3,1,2]) * sum([1,3,1,2]) = 1 * 7 = 7
The sum of all the total strengths is 1 + 9 + 1 + 4 + 4 + 4 + 3 + 5 + 6 + 7 = 44.

Example 2:

Input: strength = [5,4,6]
Output: 213
Explanation: The following are all the contiguous groups of wizards:
- [5] from [5,4,6] has a total strength of min([5]) * sum([5]) = 5 * 5 = 25
- [4] from [5,4,6] has a total strength of min([4]) * sum([4]) = 4 * 4 = 16
- [6] from [5,4,6] has a total strength of min([6]) * sum([6]) = 6 * 6 = 36
- [5,4] from [5,4,6] has a total strength of min([5,4]) * sum([5,4]) = 4 * 9 = 36
- [4,6] from [5,4,6] has a total strength of min([4,6]) * sum([4,6]) = 4 * 10 = 40
- [5,4,6] from [5,4,6] has a total strength of min([5,4,6]) * sum([5,4,6]) = 4 * 15 = 60
The sum of all the total strengths is 25 + 16 + 36 + 36 + 40 + 60 = 213.

Constraints:

    1 <= strength.length <= 10^5
    1 <= strength[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let strength = strength.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let n = strength.len();
        let n_i64 = strength.len() as i64;

        let mut ps_l = vec![0; strength.len() + 1];
        let mut pm_l = vec![0; strength.len() + 1];

        for i in 0..n {
            ps_l[i + 1] = (ps_l[i] + strength[i]) % MOD;
            let i_64 = i as i64;
            pm_l[i + 1] = (pm_l[i] + (i_64 + 1) * strength[i]) % MOD;
        }

        let mut ps_r = vec![0; strength.len() + 1];
        let mut pm_r = vec![0; strength.len() + 1];

        for i in (0..n).rev() {
            ps_r[i] = (ps_r[i + 1] + strength[i]) % MOD;
            let i_64 = i as i64;
            pm_r[i] = (pm_r[i + 1] + (n_i64 - i_64) * strength[i]) % MOD;
        }

        let mut stack = vec![];
        let mut ans = 0_i64;

        for right in 0..=n {
            while !stack.is_empty() && (right == n || strength[*stack.last().unwrap()] >= strength[right]) {
                let pivot = stack.pop().unwrap();
                let pivot_i64 = pivot as i64;

                let left_i64 = stack.last().map(|x| *x as i64 + 1).unwrap_or(0);
                let left = left_i64 as usize;

                let right_i64 = right as i64;

                let left_sum =
                    (MOD + pm_l[pivot + 1] - pm_l[left] - left_i64 * (ps_l[pivot + 1] - ps_l[left]) % MOD) % MOD;

                let right_sum =
                    (MOD + pm_r[pivot + 1] - pm_r[right] - (n_i64 - right_i64) * (ps_r[pivot + 1] - ps_r[right])) % MOD;

                let all_sum = (left_sum * (right_i64 - pivot_i64) + right_sum * (pivot_i64 - left_i64 + 1)) % MOD;

                ans = (ans + all_sum * strength[pivot]) % MOD;
            }
            stack.push(right);
        }
        ans as i32
    }
}

#[test]
fn test() {
    let strength = vec![1, 3, 1, 2];
    let res = 44;
    assert_eq!(Solution::total_strength(strength), res);
    let strength = vec![5, 4, 6];
    let res = 213;
    assert_eq!(Solution::total_strength(strength), res);
}
