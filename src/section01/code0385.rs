#![allow(dead_code)]

// 385. Mini Parser
// https://leetcode.com/problems/mini-parser/
// https://leetcode.cn/problems/mini-parser/
//
// Given a nested list of integers represented as a string, implement a parser to deserialize it.
//
// Each element is either an integer, or a list -- whose elements may also be integers or other lists.
//
// Note: You may assume that the string is well-formed:
//
// - String is non-empty.
// - String does not contain white spaces.
// - String contains only digits 0-9, [, - ,, ].
//
// Example 1:
//
// Input: s = "324"
// Output: 324
// Explanation: You should return a NestedInteger object which contains a single integer 324.
//
// Example 2:
//
// Input: s = "[123,[456,[789]]]"
// Output: [123,[456,[789]]]
// Explanation: Return a NestedInteger object containing a nested list with 2 elements:
//
// 1. An integer containing value 123.
// 2. A nested list containing two elements:
//     i.  An integer containing value 456.
//     ii. A nested list with one element:
//          a. An integer containing value 789.
//
// Example 3:
//
// Input: s = "[123,456,[788,799,833],[[]],10,[]]"
// Output: [123,456,[788,799,833],[[]],10,[]]
//
// Example 4:
//
// Input: s = "[]"
// Output: []
//
// Example 5:
//
// Input: s = "[-1]"
// Output: [-1]
//
// Constraints:
//
// - 1 <= s.length <= 5 * 10^4
// - s consists of digits 0-9 and characters '[', ']', '-', ','.
// - s is a valid expression of a nested list of integers.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl NestedInteger {
    fn new() -> Self {
        NestedInteger::List(Vec::new())
    }

    fn new_int(value: i32) -> Self {
        NestedInteger::Int(value)
    }

    fn is_integer(&self) -> bool {
        matches!(self, NestedInteger::Int(_))
    }

    fn get_integer(&self) -> i32 {
        match self {
            NestedInteger::Int(i) => *i,
            _ => panic!("Not an integer"),
        }
    }

    fn set_integer(&mut self, value: i32) {
        *self = NestedInteger::Int(value);
    }

    fn add(&mut self, ni: NestedInteger) {
        match self {
            NestedInteger::List(list) => list.push(ni),
            _ => panic!("Not a list"),
        }
    }

    fn get_list(&self) -> &Vec<NestedInteger> {
        match self {
            NestedInteger::List(list) => list,
            _ => panic!("Not a list"),
        }
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl std::str::FromStr for NestedInteger {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut list = Vec::new();
            let mut i = 1;
            while i < s.len() - 1 {
                let mut j = i;
                let mut depth = 0;
                while j < s.len() - 1 && (depth > 0 || &s[j..j + 1] != ",") {
                    if &s[j..j + 1] == "[" {
                        depth += 1;
                    } else if &s[j..j + 1] == "]" {
                        depth -= 1;
                    }
                    j += 1;
                }
                list.push(NestedInteger::from_str(&s[i..j])?);
                i = j + 1;
            }
            Ok(NestedInteger::List(list))
        } else {
            Ok(NestedInteger::Int(s.parse()?))
        }
    }
}

impl std::fmt::Display for NestedInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NestedInteger::Int(i) => write!(f, "{i}"),
            NestedInteger::List(list) => {
                write!(f, "[")?;
                for (i, ni) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{ni}")?;
                }
                write!(f, "]")
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        s.parse().unwrap_or_else(|_| NestedInteger::new())
    }

    pub fn deserialize2(s: String) -> NestedInteger {
        pub fn _deserialize2(s: String) -> Option<NestedInteger> {
            let mut stack = Vec::new();
            stack.push(NestedInteger::new());
            let mut i = 0;
            while i < s.len() {
                let c = s.chars().nth(i)?;
                if c == '[' {
                    stack.push(NestedInteger::new());
                } else if c == ']' {
                    let ni = stack.pop()?;
                    stack.last_mut()?.add(ni);
                } else if c == '-' || c.is_ascii_digit() {
                    let mut j = i + 1;
                    while j < s.len() && s.chars().nth(j)?.is_ascii_digit() {
                        j += 1;
                    }
                    let num = s[i..j].parse::<i32>().ok()?;
                    stack.last_mut()?.add(NestedInteger::new_int(num));
                    i = j - 1;
                }
                i += 1;
            }
            Some(stack.pop()?.get_list()[0].clone())
        }
        _deserialize2(s).unwrap_or_else(NestedInteger::new)
    }
}

#[test]
fn test() {
    let s = "324".to_string();
    let expected = NestedInteger::Int(324);
    assert_eq!(Solution::deserialize(s), expected);

    let s = "[-123,[456,[789]]]".to_string();
    let expected = NestedInteger::List(vec![
        NestedInteger::Int(-123),
        NestedInteger::List(vec![NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])]),
    ]);
    assert_eq!(Solution::deserialize(s), expected);

    let s = "[123,456,[788,799,833],[[]],10,[]]".to_string();
    let expected = NestedInteger::List(vec![
        NestedInteger::Int(123),
        NestedInteger::Int(456),
        NestedInteger::List(vec![NestedInteger::Int(788), NestedInteger::Int(799), NestedInteger::Int(833)]),
        NestedInteger::List(vec![NestedInteger::List(vec![])]),
        NestedInteger::Int(10),
        NestedInteger::List(vec![]),
    ]);
    assert_eq!(Solution::deserialize(s), expected);

    let s = "[]".to_string();
    let expected = NestedInteger::List(vec![]);
    assert_eq!(Solution::deserialize(s), expected);

    let s = "[-1]".to_string();
    let expected = NestedInteger::List(vec![NestedInteger::Int(-1)]);
    assert_eq!(Solution::deserialize(s), expected);
}
