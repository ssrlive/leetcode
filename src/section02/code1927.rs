#![allow(dead_code)]

/*

// 1927. Sum Game
// https://leetcode.com/problems/sum-game/
// https://leetcode.cn/problems/sum-game/
//
// Medium
//
// Alice and Bob take turns playing a game, with Alice starting first.

You are given a string num of even length consisting of digits and '?' characters. On each turn, a player will do the following if there is still at least one '?' in num:

    Choose an index i where num[i] == '?'.
    Replace num[i] with any digit between '0' and '9'.

The game ends when there are no more '?' characters in num.

For Bob to win, the sum of the digits in the first half of num must be equal to the sum of the digits in the second half. For Alice to win, the sums must not be equal.

    For example, if the game ended with num = "243801", then Bob wins because 2+4+3 = 8+0+1. If the game ended with num = "243803", then Alice wins because 2+4+3 != 8+0+3.

Assuming Alice and Bob play optimally, return true if Alice will win and false if Bob will win.

Example 1:

Input: num = "5023"
Output: false
Explanation: There are no moves to be made.
The sum of the first half is equal to the sum of the second half: 5 + 0 = 2 + 3.

Example 2:

Input: num = "25??"
Output: true
Explanation: Alice can replace one of the '?'s with '9' and it will be impossible for Bob to make the sums equal.

Example 3:

Input: num = "?3295???"
Output: false
Explanation: It can be proven that Bob will always win. One possible outcome is:
- Alice replaces the first '?' with '9'. num = "93295???".
- Bob replaces one of the '?' in the right half with '9'. num = "932959??".
- Alice replaces one of the '?' in the right half with '2'. num = "9329592?".
- Bob replaces the last '?' in the right half with '7'. num = "93295927".
Bob wins because 9 + 3 + 2 + 9 = 5 + 9 + 2 + 7.

Constraints:

    2 <= num.length <= 10^5
    num.length is even.
    num consists of only digits and '?'.
*/

struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        fn helper(memo: &mut (isize, isize), c: char) {
            if c == '?' {
                memo.1 += 1;
            } else {
                memo.0 += (c as u8 - b'0') as isize;
            }
        }

        let mut f = (0, 0);
        let mut b = (0, 0);
        let n = num.len();
        let s = num.chars().into_iter().collect::<Vec<char>>();
        for i in 0..n / 2 {
            helper(&mut f, s[i]);
            helper(&mut b, s[n - 1 - i]);
        }

        if b.0 < f.0 {
            if b.1 > f.1 {
                let left = b.1 - f.1;
                let mut num1 = left / 2;
                if left % 2 == 1 {
                    num1 += 1;
                }
                let num2 = left / 2;
                f.0 - b.0 < num1 * 9 || num2 * 9 < f.0 - b.0
            } else {
                true
            }
        } else if b.0 == f.0 {
            b.1 != f.1
        } else if b.1 < f.1 {
            let left = f.1 - b.1;
            let mut num = left / 2;
            if left % 2 == 1 {
                num += 1;
            }
            let num2 = left / 2;
            b.0 - f.0 < num * 9 || num2 * 9 < b.0 - f.0
        } else {
            true
        }
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("5023", false),
        ("25??", true),
        ("?3295???", false),
    ];
    for (num, expected) in cases {
        assert_eq!(Solution::sum_game(num.to_string()), expected);
    }
}