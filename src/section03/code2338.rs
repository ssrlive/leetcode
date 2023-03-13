#![allow(dead_code)]

/*

// 2338. Count the Number of Ideal Arrays
// https://leetcode.com/problems/count-the-number-of-ideal-arrays/
// https://leetcode.cn/problems/count-the-number-of-ideal-arrays/
//
// Hard
//
// You are given two integers n and maxValue, which are used to describe an ideal array.

A 0-indexed integer array arr of length n is considered ideal if the following conditions hold:

    Every arr[i] is a value from 1 to maxValue, for 0 <= i < n.
    Every arr[i] is divisible by arr[i - 1], for 0 < i < n.

Return the number of distinct ideal arrays of length n. Since the answer may be very large, return it modulo 109 + 7.

Example 1:

Input: n = 2, maxValue = 5
Output: 10
Explanation: The following are the possible ideal arrays:
- Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
- Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
- Arrays starting with the value 3 (1 array): [3,3]
- Arrays starting with the value 4 (1 array): [4,4]
- Arrays starting with the value 5 (1 array): [5,5]
There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.

Example 2:

Input: n = 5, maxValue = 3
Output: 11
Explanation: The following are the possible ideal arrays:
- Arrays starting with the value 1 (9 arrays):
   - With no other distinct values (1 array): [1,1,1,1,1]
   - With 2nd distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
   - With 2nd distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
- Arrays starting with the value 2 (1 array): [2,2,2,2,2]
- Arrays starting with the value 3 (1 array): [3,3,3,3,3]
There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.

Constraints:

    2 <= n <= 10^4
    1 <= maxValue <= 10^4
*/

struct Solution;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        use std::collections::HashMap;
        const MOD: i32 = 1_000_000_007;

        fn add(a: i32, b: i32) -> i32 {
            let mut r = a + b;
            if r >= MOD {
                r -= MOD;
            }
            r
        }

        fn mul(a: i32, b: i32) -> i32 {
            ((a as i64) * (b as i64) % (MOD as i64)) as i32
        }

        fn edges(max_value: usize) -> (Vec<i32>, Vec<Vec<(i32, i32)>>) {
            let mut divisor = vec![1; max_value + 1];
            for i in 2..=max_value {
                if divisor[i] != 1 {
                    continue;
                }
                for j in (i..=max_value).step_by(i) {
                    divisor[j] = i;
                }
            }
            let mut states = HashMap::new();
            let mut state_id = vec![0; max_value + 1];
            for (i, state_id_i) in state_id.iter_mut().enumerate().skip(1) {
                let mut state = Vec::new();
                let mut n = i;
                while n != 1 {
                    let p = divisor[n];
                    let mut d = 0;
                    while n % p == 0 {
                        d += 1;
                        n /= p;
                    }
                    state.push(d);
                }
                state.sort();
                let size = states.len() as i32;
                let id = *states.entry(state).or_insert(size);
                *state_id_i = id;
            }
            let mut some_val_for_state = vec![0; states.len()];
            for i in (1..=max_value).rev() {
                let s = state_id[i] as usize;
                some_val_for_state[s] = i;
            }
            let mut edges = vec![HashMap::new(); states.len()];
            for i in 1..=max_value {
                for j in (2 * i..=max_value).step_by(i) {
                    let s = state_id[j] as usize;
                    if j == some_val_for_state[s] {
                        *edges[s].entry(state_id[i]).or_insert(0) += 1;
                    }
                }
            }
            let edges = edges
                .into_iter()
                .map(|map| map.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            (state_id, edges)
        }

        let max_value = max_value as usize;
        let (state_id, edges) = edges(max_value);
        let mut dp = vec![0; edges.len()];
        dp[0] = 1;
        for _ in 0..n {
            for i in (0..dp.len()).rev() {
                for &(to, cnt) in edges[i].iter() {
                    dp[i] = add(dp[i], mul(dp[to as usize], cnt));
                }
            }
        }
        let mut result = 0;
        for i in 1..=max_value {
            result = add(result, dp[state_id[i] as usize]);
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::ideal_arrays(2, 5), 10);
    assert_eq!(Solution::ideal_arrays(5, 3), 11);
}
