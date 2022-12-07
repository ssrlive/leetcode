#![allow(dead_code)]

// 679. 24 Game
// https://leetcode.com/problems/24-game/
//
// You are given an integer array cards of length 4. You have four cards, each
// containing a number in the range [1, 9]. You should arrange the numbers on
// these cards in a mathematical expression using the operators ['+', '-', '*',
// '/'] and the parentheses '(' and ')' to get the value 24.
//
// You are restricted with the following rules:
//
// - The division operator '/' represents real division, not integer division.
// - For example, 4 / (1 - 2 / 3) = 4 / (1 / 3) = 12.
// - Every operation done is between two numbers. In particular, we cannot use
//   '-' as a unary operator. For example, if cards = [1, 1, 1, 1], the
//   expression "-1 - 1 - 1 - 1" is not allowed.
// - You cannot concatenate numbers together. For example, if cards = [1, 2, 1,
//   2], the expression "12 + 12" is not valid.
//
// Return true if you can get such expression that evaluates to 24, and false
// otherwise.
//
// Example 1:
//
// Input: cards = [4,1,8,7]
// Output: true
// Explanation: (8-4) * (7-1) = 24
//
// Example 2:
//
// Input: cards = [1,2,1,2]
// Output: false
//
// Constraints:
//
// - cards.length == 4
// - 1 <= cards[i] <= 9
//

struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let ops = vec!['+', '-', '*', '/'];

        fn calc(m: f64, op: char, n: f64) -> f64 {
            match op {
                '+' => m + n,
                '-' => m - n,
                '*' => m * n,
                '/' => m / n,
                _ => panic!("operation not supported"),
            }
        }

        // help to evaluate an expression by using all operators
        fn eval<F>(ops: &[char], f: F) -> bool
        where
            F: Fn(char, char, char) -> f64,
        {
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        let val = f(ops[i], ops[j], ops[k]);
                        if (val - 24.0).abs() < 0.001 {
                            return true;
                        }
                    }
                }
            }
            false
        }

        // find all unique permutations of a integer vector.
        fn all_permutations(items: Vec<i32>) -> Vec<Vec<i32>> {
            let mut result_hash = HashSet::new();
            if items.len() == 1 {
                return vec![items];
            }
            for i in 0..items.len() {
                let item = items[i];
                let rest = items
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &r)| if index != i { Some(r) } else { None })
                    .collect::<Vec<i32>>();
                let mut rest_permutations = all_permutations(rest);
                rest_permutations.iter_mut().for_each(|v| {
                    let mut new_vec = vec![item];
                    new_vec.append(v);
                    result_hash.insert(new_vec);
                });
            }
            result_hash.into_iter().collect::<Vec<Vec<i32>>>()
        }

        let permutations = all_permutations(cards.into_iter().collect::<Vec<i32>>());
        // println!("Permutations: {:?}", permutations);
        for permutation in permutations {
            // (0 op 1) op (2 op 3)
            if eval(&ops, |op_i, op_j, op_k| {
                calc(
                    calc(permutation[0] as f64, op_i, permutation[1] as f64),
                    op_j,
                    calc(permutation[2] as f64, op_k, permutation[3] as f64),
                )
            }) {
                return true;
            }

            // ((0 op 1) op 2) op 3
            if eval(&ops, |op_i, op_j, op_k| {
                calc(
                    calc(
                        calc(permutation[0] as f64, op_i, permutation[1] as f64),
                        op_j,
                        permutation[2] as f64,
                    ),
                    op_k,
                    permutation[3] as f64,
                )
            }) {
                return true;
            }
            // (0 op (1 op 2)) op 3
            if eval(&ops, |op_i, op_j, op_k| {
                calc(
                    calc(
                        permutation[0] as f64,
                        op_i,
                        calc(permutation[1] as f64, op_j, permutation[2] as f64),
                    ),
                    op_k,
                    permutation[3] as f64,
                )
            }) {
                return true;
            }
            // 0 op ((1 op 2) op 3)
            if eval(&ops, |op_i, op_j, op_k| {
                calc(
                    permutation[0] as f64,
                    op_i,
                    calc(
                        calc(permutation[1] as f64, op_j, permutation[2] as f64) as f64,
                        op_k,
                        permutation[3] as f64,
                    ),
                )
            }) {
                return true;
            }
            // 0 op (1 op (2 op 3))
            if eval(&ops, |op_i, op_j, op_k| {
                calc(
                    permutation[0] as f64,
                    op_i,
                    calc(
                        permutation[1] as f64,
                        op_j,
                        calc(permutation[2] as f64, op_k, permutation[3] as f64),
                    ),
                )
            }) {
                return true;
            }
        }

        false
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 1, 8, 7], true),
        (vec![1, 2, 1, 2], false),
        (vec![1, 3, 2, 6], true),
    ];
    for (cards, expected) in cases {
        assert_eq!(Solution::judge_point24(cards), expected);
    }
}
