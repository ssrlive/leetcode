#![allow(dead_code)]

// 155. Min Stack
// https://leetcode.com/problems/min-stack/
// https://leetcode.cn/problems/min-stack/
//
// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
//
// Implement the MinStack class:
// - MinStack() initializes the stack object.
// - void push(val) pushes the element val onto the stack.
// - void pop() removes the element on the top of the stack.
// - int top() gets the top element of the stack.
// - int getMin() retrieves the minimum element in the stack.
//
// You must implement a solution with O(1) time complexity for each function.
//

struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if let Some(&min) = self.min.last() {
            if val <= min {
                self.min.push(val);
            }
        } else {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if let Some(&min) = self.min.last() {
                if val == min {
                    self.min.pop();
                }
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[test]
fn test_min_stack() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}
