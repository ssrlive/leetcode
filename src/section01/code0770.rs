#![allow(dead_code)]

// 770. Basic Calculator IV
// https://leetcode.com/problems/basic-calculator-iv/
// https://leetcode.cn/problems/basic-calculator-iv/
//
// Given an expression such as expression = "e + 8 - a + 5" and an evaluation map such as {"e": 1} (given in terms of evalvars = ["e"] and evalints = [1]),
// return a list of tokens representing the simplified expression, such as ["-1*a","14"]
//
// - An expression alternates chunks and symbols, with a space separating each chunk and symbol.
// - A chunk is either an expression in parentheses, a variable, or a non-negative integer.
// - A variable is a string of lowercase letters (not including digits.) Note that variables can be multiple letters,
//   and note that variables never have a leading coefficient or unary operator like "2x" or "-x".
//
// Expressions are evaluated in the usual order: brackets first, then multiplication, then addition and subtraction.
//
// - For example, expression = "1 + 2 * 3" has an answer of ["7"].
//
// The format of the output is as follows:
//
// - For each term of free variables with a non-zero coefficient, we write the free variables within a term in sorted order lexicographically.
//   - For example, we would never write a term like "b*a*c", only "a*b*c".
// - Terms have degrees equal to the number of free variables being multiplied, counting multiplicity. We write the largest degree terms of our answer first,
//   breaking ties by lexicographic order ignoring the leading coefficient of the term.
//   - For example, "a*a*b*c" has degree 4.
// - The leading coefficient of the term is placed directly to the left with an asterisk separating it from the variables
//   (if they exist.) A leading coefficient of 1 is still printed.
// - An example of a well-formatted answer is ["-2*a*a*a", "3*a*a*b", "3*b*b", "4*a", "5*c", "-6"].
// - Terms (including constant terms) with coefficient 0 are not included.
//   - For example, an expression of "0" has an output of [].
//
// Note: You may assume that the given expression is always valid. All intermediate results will be in the range of [-2^31, 2^31 - 1].
//
// Example 1:
//
// Input: expression = "e + 8 - a + 5", evalvars = ["e"], evalints = [1]
// Output: ["-1*a","14"]
//
// Example 2:
//
// Input: expression = "e - 8 + temperature - pressure", evalvars = ["e", "temperature"], evalints = [1, 12]
// Output: ["-1*pressure","5"]
//
// Example 3:
//
// Input: expression = "(e + 8) * (e - 8)", evalvars = [], evalints = []
// Output: ["1*e*e","-64"]
//
// Constraints:
//
// - 1 <= expression.length <= 250
// - expression consists of lowercase English letters, digits, '+', '-', '*', '(', ')', ' '.
// - expression does not contain any leading or trailing spaces.
// - All the tokens in expression are separated by a single space.
// - 0 <= evalvars.length <= 100
// - 1 <= evalvars[i].length <= 20
// - evalvars[i] consists of lowercase English letters.
// - evalints.length == evalvars.length
// - -100 <= evalints[i] <= 100
//

struct Solution;

impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        fn sum_map(st_map: Vec<HashMap<String, i32>>) -> HashMap<String, i32> {
            let mut res = HashMap::new();
            for num_map in st_map {
                for (k, v) in num_map {
                    *res.entry(k).or_insert(0) += v;
                }
            }
            res
        }

        fn multiply(num1: &HashMap<String, i32>, num2: &HashMap<String, i32>) -> HashMap<String, i32> {
            let mut res = HashMap::new();
            for (k1, &v1) in num1 {
                for (k2, &v2) in num2 {
                    if v1 == 0 || v2 == 0 {
                        continue;
                    }
                    let mut mul_key = vec![];
                    if !k1.is_empty() {
                        mul_key.extend(k1.split('*'));
                    }
                    if !k2.is_empty() {
                        mul_key.extend(k2.split('*'));
                    }
                    mul_key.sort();
                    let key = mul_key.join("*");
                    *res.entry(key).or_insert(0) += v1 * v2;
                }
            }
            res
        }

        fn dfs(s: &mut VecDeque<String>, eval_map: &HashMap<&String, i32>) -> Option<HashMap<String, i32>> {
            let mut st = vec![];

            let mut num = HashMap::new();
            num.insert("".to_string(), 0);

            let mut sign = "+".to_string();
            while !s.is_empty() {
                let char = s.pop_front()?;
                if char.parse::<i32>().is_ok() || eval_map.contains_key(&char) {
                    let number = if eval_map.contains_key(&char) {
                        *eval_map.get(&char)?
                    } else {
                        char.parse::<i32>().ok()?
                    };
                    num.clear();
                    num.insert("".to_string(), number);
                } else if char.chars().all(|c| c.is_alphabetic()) {
                    num.clear();
                    num.insert(char.clone(), 1);
                }
                if char == "(" {
                    num = dfs(s, eval_map)?;
                }
                if s.is_empty() || "+-*()".contains(&char[..]) {
                    if sign == "+" {
                        st.push(num.clone());
                    }
                    if sign == "-" {
                        for (_, v) in num.iter_mut() {
                            *v = -*v;
                        }
                        st.push(num.clone());
                    }
                    if sign == "*" {
                        let v = multiply(&st.pop()?, &num);
                        st.push(v);
                    }
                    sign.clone_from(&char);
                }
                if char == ")" {
                    break;
                }
            }
            Some(sum_map(st))
        }

        fn _basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Option<Vec<String>> {
            let mut eval_map = HashMap::new();
            for (e, v) in evalvars.iter().zip(evalints.iter()) {
                eval_map.insert(e, *v);
            }
            let expression = expression.replace('(', "( ").replace(')', " )");
            let mut s: VecDeque<String> = expression.split(' ').map(|s| s.to_string()).collect();

            let res_map = dfs(&mut s, &eval_map)?;

            let mut res = vec![];
            let mut f_num = vec![];
            let mut keys: Vec<String> = res_map.keys().map(|s| s.to_string()).collect();
            keys.sort_by(|a, b| {
                let a_len = a.split('*').count();
                let b_len = b.split('*').count();
                if a_len == b_len {
                    a.cmp(b)
                } else {
                    b_len.cmp(&a_len)
                }
            });
            for k in keys.iter() {
                let val = res_map[k];
                if val == 0 {
                    continue;
                }
                if k.is_empty() {
                    f_num = vec![val.to_string()];
                } else {
                    res.push(format!("{val}*{k}"));
                }
            }
            res.extend(f_num);
            Some(res)
        }

        _basic_calculator_iv(expression, evalvars, evalints).unwrap_or_default()
    }
}

#[test]
fn test() {
    let expression = "e + 8 - a + 5".to_string();
    let evalvars = vec!["e".to_string()];
    let evalints = vec![1];
    let output = vec!["-1*a".to_string(), "14".to_string()];
    assert_eq!(Solution::basic_calculator_iv(expression, evalvars, evalints), output);

    let expression = "e - 8 + temperature - pressure".to_string();
    let evalvars = vec!["e".to_string(), "temperature".to_string()];
    let evalints = vec![1, 12];
    let output = vec!["-1*pressure".to_string(), "5".to_string()];
    assert_eq!(Solution::basic_calculator_iv(expression, evalvars, evalints), output);

    let expression = "(e + 8) * (e - 8)".to_string();
    let evalvars = vec![];
    let evalints = vec![];
    let output = vec!["1*e*e".to_string(), "-64".to_string()];
    assert_eq!(Solution::basic_calculator_iv(expression, evalvars, evalints), output);
}
