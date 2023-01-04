#![allow(dead_code)]

// 946. Validate Stack Sequences
// https://leetcode.com/problems/validate-stack-sequences/
// https://leetcode.cn/problems/validate-stack-sequences/
//
// Given two integer arrays pushed and popped each with distinct values, return true if this could have been the result
// of a sequence of push and pop operations on an initially empty stack, or false otherwise.
//
// Example 1:
//
// Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
// Output: true
// Explanation: We might do the following sequence:
// push(1), push(2), push(3), push(4),
// pop() -> 4,
// push(5),
// pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
//
// Example 2:
//
// Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
// Output: false
// Explanation: 1 cannot be popped before 2.
//
// Constraints:
//
// - 1 <= pushed.length <= 1000
// - 0 <= pushed[i] <= 1000
// - All the elements of pushed are unique.
// - popped.length == pushed.length
// popped is a permutation of pushed.
//

struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut i = 0;
        for &n in &pushed {
            stack.push(n);
            while let Some(&last) = stack.last() {
                if last == popped[i] {
                    stack.pop();
                    i += 1;
                } else {
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1], true),
        (vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2], false),
    ];
    for (pushed, popped, expected) in cases {
        assert_eq!(Solution::validate_stack_sequences(pushed, popped), expected);
    }
}
