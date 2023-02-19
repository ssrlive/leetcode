#![allow(dead_code)]

// 1735. Count Ways to Make Array With Product
// https://leetcode.com/problems/count-ways-to-make-array-with-product/
// https://leetcode.cn/problems/count-ways-to-make-array-with-product/
//
// Hard
//
// You are given a 2D integer array, queries. For each queries[i], where queries[i] = [ni, ki], find the number of different ways you can place positive integers into an array of size ni such that the product of the integers is ki. As the number of ways may be too large, the answer to the ith query is the number of ways modulo 109 + 7.
//
// Return an integer array answer where answer.length == queries.length, and answer[i] is the answer to the ith query.
//
// Example 1:
//
// Input: queries = [[2,6],[5,1],[73,660]]
// Output: [4,1,50734910]
// Explanation: Each query is independent.
// [2,6]: There are 4 ways to fill an array of size 2 that multiply to 6: [1,6], [2,3], [3,2], [6,1].
// [5,1]: There is 1 way to fill an array of size 5 that multiply to 1: [1,1,1,1,1].
// [73,660]: There are 1050734917 ways to fill an array of size 73 that multiply to 660. 1050734917 modulo 109 + 7 = 50734910.
//
// Example 2:
//
// Input: queries = [[1,1],[2,2],[3,3],[4,4],[5,5]]
// Output: [1,2,3,10,5]
//
// Constraints:
//
// -    1 <= queries.length <= 10^4
// -    1 <= ni, ki <= 10^4

struct Solution;

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::*;
        const MOD: usize = 1_000_000_007;

        fn mod_pow(mut a: usize, mut n: usize) -> usize {
            let mut res = 1;
            while n > 0 {
                if n & 1 == 1 {
                    res = res * a % MOD;
                }
                a = a * a % MOD;
                n >>= 1;
            }
            res
        }

        fn mod_inv(a: usize) -> usize {
            mod_pow(a, MOD - 2)
        }

        let mut result = vec![];
        for arr in queries {
            let n = arr[0] as usize;
            let mut k = arr[1] as usize;
            let mut i = 2;
            let mut map = HashMap::new();
            while i * i <= k {
                if k % i == 0 {
                    *map.entry(i).or_insert(0) += 1;
                    k /= i;
                } else {
                    i += 1;
                }
            }
            if 1 < k {
                *map.entry(k).or_insert(0) += 1;
            }
            let mut temp = 1;
            for (_, choose_k) in map {
                let choose_n = n + choose_k - 1;
                let mut cv = 1;
                let mut pv = 1;
                for i in (choose_n - choose_k + 1)..=choose_n {
                    cv *= i;
                    cv %= MOD;
                }
                for i in 1..=choose_k {
                    pv *= i;
                    pv %= MOD;
                }
                temp *= cv * mod_inv(pv) % MOD;
                temp %= MOD;
            }
            result.push(temp as i32);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![2, 6], vec![5, 1], vec![73, 660]], vec![4, 1, 50734910]),
        (
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
            vec![1, 2, 3, 10, 5],
        ),
    ];
    for (queries, expected) in cases {
        assert_eq!(Solution::ways_to_fill_array(queries), expected);
    }
}
