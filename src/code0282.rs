#![allow(dead_code)]

// 282. Expression Add Operators
// https://leetcode.com/problems/expression-add-operators/
//
// Given a string num that contains only digits and an integer target, return all
// possibilities to add the binary operators '+', '-', or '*' between the digits
// of num so that the resultant expression evaluates to the target value.
//
// Example 1:
//
// Input: num = "123", target = 6
// Output: ["1*2*3","1+2+3"]
//
// Example 2:
//
// Input: num = "232", target = 8
// Output: ["2*3+2","2+3*2"]
//
// Example 3:
//
// Input: num = "105", target = 5
// Output: ["1*0+5","10-5"]
//
// Example 4:
//
// Input: num = "00", target = 0
// Output: ["0*0","0+0","0-0"]
//
// Example 5:
//
// Input: num = "3456237490", target = 9191
// Output: []
//
// Constraints:
//
// 1 <= num.length <= 10
// num consists of only digits.
// -2^31 <= target <= 2^31 - 1

struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let v = num.bytes().map(|u| (u - b'0') as i64).collect::<Vec<_>>();
        let mut answer = Vec::new();
        Self::backtrack(&mut answer, &mut Vec::new(), &v, 0, 0, target as i64);
        answer
    }
    fn backtrack(
        answer: &mut Vec<String>,
        ops: &mut Vec<String>,
        v: &[i64],
        val: i64,
        last: i64,
        target: i64,
    ) {
        if v.is_empty() {
            if val == target {
                answer.push(ops[1..].join(""));
            }
            return;
        }
        let mut n = 0;
        for (i, &d) in v.iter().enumerate() {
            n = n * 10 + d;
            ops.push(String::from("+"));
            ops.push(n.to_string());
            Self::backtrack(answer, ops, &v[i + 1..], val + n, n, target);
            ops.pop();
            ops.pop();
            if !ops.is_empty() {
                ops.push(String::from("-"));
                ops.push(n.to_string());
                Self::backtrack(answer, ops, &v[i + 1..], val - n, -n, target);
                ops.pop();
                ops.pop();
                ops.push(String::from("*"));
                ops.push(n.to_string());
                Self::backtrack(
                    answer,
                    ops,
                    &v[i + 1..],
                    val - last + last * n,
                    last * n,
                    target,
                );
                ops.pop();
                ops.pop();
            }
            if v[0] == 0 {
                break;
            }
        }
    }
}

#[test]
fn test() {
    let num = "123".to_string();
    let target = 6;
    let result = vec!["1*2*3", "1+2+3"];
    let mut v = Solution::add_operators(num, target);
    v.sort();
    assert_eq!(v, result);

    let num = "232".to_string();
    let target = 8;
    let result = vec!["2*3+2", "2+3*2"];
    let mut v = Solution::add_operators(num, target);
    v.sort();
    assert_eq!(v, result);

    let num = "105".to_string();
    let target = 5;
    let result = vec!["1*0+5", "10-5"];
    assert_eq!(Solution::add_operators(num, target), result);

    let num = "00".to_string();
    let target = 0;
    let result = vec!["0*0", "0+0", "0-0"];
    let mut v = Solution::add_operators(num, target);
    v.sort();
    assert_eq!(v, result);

    let num = "3456237490".to_string();
    let target = 9191;
    let result: Vec<&str> = vec![];
    assert_eq!(Solution::add_operators(num, target), result);
}
