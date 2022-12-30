#![allow(dead_code)]

// 895. Maximum Frequency Stack
// https://leetcode.com/problems/maximum-frequency-stack/
// https://leetcode.cn/problems/maximum-frequency-stack/
//
// Design a stack-like data structure to push elements to the stack and pop the most frequent element from the stack.
//
// Implement the FreqStack class:
//
// FreqStack() constructs an empty frequency stack.
// void push(int val) pushes an integer val onto the top of the stack.
// int pop() removes and returns the most frequent element in the stack.
// If there is a tie for the most frequent element, the element closest to the stack's top is removed and returned.
//
// Example 1:
//
// Input
// ["FreqStack", "push", "push", "push", "push", "push", "push", "pop", "pop", "pop", "pop"]
// [[], [5], [7], [5], [7], [4], [5], [], [], [], []]
// Output
// [null, null, null, null, null, null, null, 5, 7, 5, 4]
//
// Explanation
// FreqStack freqStack = new FreqStack();
// freqStack.push(5); // The stack is [5]
// freqStack.push(7); // The stack is [5,7]
// freqStack.push(5); // The stack is [5,7,5]
// freqStack.push(7); // The stack is [5,7,5,7]
// freqStack.push(4); // The stack is [5,7,5,7,4]
// freqStack.push(5); // The stack is [5,7,5,7,4,5]
// freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,5,7,4].
// freqStack.pop();   // return 7, as 5 and 7 is the most frequent, but 7 is closest to the top. The stack becomes [5,7,5,4].
// freqStack.pop();   // return 5, as 5 is the most frequent. The stack becomes [5,7,4].
// freqStack.pop();   // return 4, as 4, 5 and 7 is the most frequent, but 4 is closest to the top. The stack becomes [5,7].
//
// Constraints:
//
// - 0 <= val <= 10^9
// - At most 2 * 10^4 calls will be made to push and pop.
// - It is guaranteed that there will be at least one element in the stack before calling pop.
//

use std::collections::{HashMap, VecDeque};

struct FreqStack {
    freq: HashMap<i32, i32>,
    stack: HashMap<i32, VecDeque<i32>>,
    max_freq: i32,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            stack: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let freq = self.freq.entry(val).or_insert(0);
        *freq += 1;
        self.max_freq = self.max_freq.max(*freq);
        self.stack.entry(*freq).or_default().push_back(val);
    }

    fn pop(&mut self) -> i32 {
        let val = self.stack.get_mut(&self.max_freq).unwrap().pop_back().unwrap();
        let freq = self.freq.get_mut(&val).unwrap();
        *freq -= 1;
        if *freq == 0 {
            self.freq.remove(&val);
        }
        if self.stack.get(&self.max_freq).unwrap().is_empty() {
            self.stack.remove(&self.max_freq);
            self.max_freq -= 1;
        }
        val
    }
}

#[test]
fn test() {
    let mut freq_stack = FreqStack::new();
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(4);
    freq_stack.push(5);
    assert_eq!(freq_stack.pop(), 5);
    assert_eq!(freq_stack.pop(), 7);
    assert_eq!(freq_stack.pop(), 5);
    assert_eq!(freq_stack.pop(), 4);
}
