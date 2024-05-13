#![allow(dead_code)]

// 38. Count and Say
// https://leetcode.com/problems/count-and-say/
// https://leetcode.cn/problems/count-and-say/
//
// The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
//
// - countAndSay(1) = "1"
// - countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a different digit string.
//
// To determine how you "say" a digit string, split it into the minimal number of substrings such that each substring contains exactly one unique digit. Then for each substring, say the number of digits, then say the digit. Finally, concatenate every said digit.
//
// For example, the saying and conversion for digit string "3322251":
//
// Given a positive integer n, return the nth term of the count-and-say sequence.
//
// Example 1:
//
// Input: n = 1
// Output: "1"
// Explanation: This is the base case.
//
// Example 2:
//
// Input: n = 4
// Output: "1211"
// Explanation:
// countAndSay(1) = "1"
// countAndSay(2) = say "1" = one 1 = "11"
// countAndSay(3) = say "11" = two 1's = "21"
// countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
//
// Constraints:
//
// - 1 <= n <= 30
//

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut v = vec!['1'];
        for _ in 1..n {
            let mut last_char = ' ';
            let mut count = 0;
            let mut ans = Vec::new();
            for (idx, &c) in v.iter().enumerate() {
                if idx == 0 {
                    last_char = c;
                    count += 1;
                } else if last_char == c {
                    count += 1;
                } else {
                    ans.push(char::from_digit(count, 10).unwrap());
                    ans.push(last_char);
                    last_char = c;
                    count = 1;
                }
            }
            ans.push(char::from_digit(count, 10).unwrap());
            ans.push(last_char);
            v.clone_from(&ans);
        }
        v.into_iter().collect()
    }
}

#[test]
fn test_count_and_say() {
    assert_eq!(Solution::count_and_say(1), "1");
    assert_eq!(Solution::count_and_say(2), "11");
    assert_eq!(Solution::count_and_say(3), "21");
    assert_eq!(Solution::count_and_say(4), "1211");
    assert_eq!(Solution::count_and_say(5), "111221");
    assert_eq!(Solution::count_and_say(6), "312211");
    assert_eq!(Solution::count_and_say(7), "13112221");
    assert_eq!(Solution::count_and_say(8), "1113213211");
    assert_eq!(Solution::count_and_say(9), "31131211131221");
    assert_eq!(Solution::count_and_say(10), "13211311123113112211");
}
