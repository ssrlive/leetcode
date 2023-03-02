#![allow(dead_code)]

/*

// 2024. Maximize the Confusion of an Exam
// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
// https://leetcode.cn/problems/maximize-the-confusion-of-an-exam/
//
// Medium
//
// A teacher is writing a test with n true/false questions, with 'T' denoting true and 'F' denoting false. He wants to confuse the students by maximizing the number of consecutive questions with the same answer (multiple trues or multiple falses in a row).

You are given a string answerKey, where answerKey[i] is the original answer to the ith question. In addition, you are given an integer k, the maximum number of times you may perform the following operation:

    Change the answer key for any question to 'T' or 'F' (i.e., set answerKey[i] to 'T' or 'F').

Return the maximum number of consecutive 'T's or 'F's in the answer key after performing the operation at most k times.

Example 1:

Input: answerKey = "TTFF", k = 2
Output: 4
Explanation: We can replace both the 'F's with 'T's to make answerKey = "TTTT".
There are four consecutive 'T's.

Example 2:

Input: answerKey = "TFFT", k = 1
Output: 3
Explanation: We can replace the first 'T' with an 'F' to make answerKey = "FFFT".
Alternatively, we can replace the second 'T' with an 'F' to make answerKey = "TFFF".
In both cases, there are three consecutive 'F's.

Example 3:

Input: answerKey = "TTFTTFTT", k = 1
Output: 5
Explanation: We can replace the first 'F' to make answerKey = "TTTTTFTT"
Alternatively, we can replace the second 'F' to make answerKey = "TTFTTTTT".
In both cases, there are five consecutive 'T's.

Constraints:

    n == answerKey.length
    1 <= n <= 5 * 10^4
    answerKey[i] is either 'T' or 'F'
    1 <= k <= n
*/

struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut t_k = k;
        let mut f_k = k;
        let mut t_i = 0;
        let mut f_i = 0;
        let chars = answer_key.as_bytes();
        for &ch in answer_key.as_bytes() {
            if ch == b'T' {
                t_k -= 1;
            } else {
                f_k -= 1;
            }
            if t_k < 0 {
                if chars[t_i] == b'T' {
                    t_k += 1;
                }
                t_i += 1;
            }
            if f_k < 0 {
                if chars[f_i] == b'F' {
                    f_k += 1;
                }
                f_i += 1;
            }
        }
        let n = answer_key.len();
        (n - t_i).max(n - f_i) as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("TTFF", 2, 4),
        ("TFFT", 1, 3),
        ("TTFTTFTT", 1, 5),
    ];
    for (answer_key, k, expected) in cases {
        assert_eq!(Solution::max_consecutive_answers(answer_key.to_string(), k), expected);
    }
}
