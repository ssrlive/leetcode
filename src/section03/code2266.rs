#![allow(dead_code)]

/*

// 2266. Count Number of Texts
// https://leetcode.com/problems/count-number-of-texts/
// https://leetcode.cn/problems/count-number-of-texts/
//
// Medium
//
// Alice is texting Bob using her phone. The mapping of digits to letters is shown in the figure below.

In order to add a letter, Alice has to press the key of the corresponding digit i times, where i is the position of the letter in the key.

    For example, to add the letter 's', Alice has to press '7' four times. Similarly, to add the letter 'k', Alice has to press '5' twice.
    Note that the digits '0' and '1' do not map to any letters, so Alice does not use them.

However, due to an error in transmission, Bob did not receive Alice's text message but received a string of pressed keys instead.

    For example, when Alice sent the message "bob", Bob received the string "2266622".

Given a string pressedKeys representing the string received by Bob, return the total number of possible text messages Alice could have sent.

Since the answer may be very large, return it modulo 109 + 7.

Example 1:

Input: pressedKeys = "22233"
Output: 8
Explanation:
The possible text messages Alice could have sent are:
"aaadd", "abdd", "badd", "cdd", "aaae", "abe", "bae", and "ce".
Since there are 8 possible messages, we return 8.

Example 2:

Input: pressedKeys = "222222222222222222222222222222222222"
Output: 82876089
Explanation:
There are 2082876103 possible text messages Alice could have sent.
Since we need to return the answer modulo 109 + 7, we return 2082876103 % (109 + 7) = 82876089.

Constraints:

    1 <= pressedKeys.length <= 10^5
    pressedKeys only consists of digits from '2' - '9'.
*/

struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        const MOD: usize = 1_000_000_007;

        fn helper(pair: (usize, bool)) -> usize {
            let n = pair.0;
            let padding = if pair.1 { 4 } else { 3 };
            let mut memo = vec![vec![0usize; n + 1]; padding];
            memo[0][0] = 1;
            for i in 0..n {
                let ci = i % padding;
                let ni = (i + 1) % padding;
                let limit = std::cmp::min(n, i + padding);
                for j in i + 1..=limit {
                    memo[ni][j] = memo[ci][j] + memo[ci][i];
                    if memo[ni][j] > MOD {
                        memo[ni][j] %= MOD;
                    }
                }
            }
            memo[n % padding][n]
        }

        let s = pressed_keys.chars().collect::<Vec<char>>();
        let mut stack = vec![(1, false)];
        let mut last = s[0];
        if last == '7' || last == '9' {
            stack[0].1 = true;
        }
        for &s_i in s.iter().skip(1) {
            if last != s_i {
                last = s_i;
                if last == '7' || last == '9' {
                    stack.push((1, true));
                } else {
                    stack.push((1, false));
                }
            } else {
                let li = stack.len() - 1;
                stack[li].0 += 1;
            }
        }
        let mut result = 1;
        for v in stack {
            let pattern = helper(v);
            result *= pattern;
            result %= MOD;
        }
        result as i32
    }
}

#[test]
fn test() {
    let cases = vec![("22233", 8), ("222222222222222222222222222222222222", 82876089)];
    for (pressed_keys, expected) in cases {
        assert_eq!(Solution::count_texts(pressed_keys.to_string()), expected);
    }
}
