#![allow(dead_code)]

// 838. Push Dominoes
// https://leetcode.com/problems/push-dominoes/
// https://leetcode.cn/problems/push-dominoes/
//
// There are n dominoes in a line, and we place each domino vertically upright. In the beginning,
// we simultaneously push some of the dominoes either to the left or to the right.
//
// After each second, each domino that is falling to the left pushes the adjacent domino on the left.
// Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
//
// When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
//
// For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
//
// You are given a string dominoes representing the initial state where:
//
// - dominoes[i] = 'L', if the i-th domino has been pushed to the left,
// - dominoes[i] = 'R', if the i-th domino has been pushed to the right, and
// - dominoes[i] = '.', if the i-th domino has not been pushed.
//
// Return a string representing the final state.
//
// Example 1:
//
// Input: dominoes = "RR.L"
// Output: "RR.L"
// Explanation: The first domino expends no additional force on the second domino.
//
// Example 2:
//
// Input: dominoes = ".L.R...LR..L.."
// Output: "LL.RR.LLRRLL.."
//
// Constraints:
//
// - n == dominoes.length
// - 1 <= n <= 10^5
// - dominoes[i] is either 'L', 'R', or '.'.
//

struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut ste: Vec<i32> = vec![0; dominoes.len()];
        let mut curr: i32 = 0;
        for (idx, c) in dominoes.char_indices() {
            match c {
                'R' => curr = i32::MAX,
                'L' => curr = 0,
                _ => curr = i32::max(0, curr - 1),
            }
            ste[idx] = curr
        }
        curr = 0;
        for (idx, c) in dominoes.char_indices().rev() {
            match c {
                'L' => curr = i32::MAX,
                'R' => curr = 0,
                _ => curr = i32::max(0, curr - 1),
            }
            ste[idx] -= curr
        }
        ste.iter()
            .map(|&x| match x.cmp(&0) {
                std::cmp::Ordering::Less => 'L',
                std::cmp::Ordering::Greater => 'R',
                std::cmp::Ordering::Equal => '.',
            })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::push_dominoes("RR.L".to_string()), "RR.L".to_string());

    let dominoes = ".L.R...LR..L..".to_string();
    assert_eq!(Solution::push_dominoes(dominoes), "LL.RR.LLRRLL..".to_string());
}
