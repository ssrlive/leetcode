#![allow(dead_code)]

// 225. Implement Stack using Queues
// https://leetcode.com/problems/implement-stack-using-queues/
// https://leetcode.cn/problems/implement-stack-using-queues/
//
// mplement a last-in-first-out (LIFO) stack using only two queues. The implemented stack
// should support all the functions of a normal stack (push, top, pop, and empty).
//
// Implement the MyStack class:
//
// - void push(int x) Pushes element x to the top of the stack.
// - int pop() Removes the element on the top of the stack and returns it.
// - int top() Returns the element on the top of the stack.
// - boolean empty() Returns true if the stack is empty, false otherwise.
//
// Notes:
//
// - You must use only standard operations of a queue, which means that only push to back,
//   peek/pop from front, size and is empty operations are valid.
// - Depending on your language, the queue may not be supported natively.
//   You may simulate a queue using a list or deque (double-ended queue)
//   as long as you use only a queue's standard operations.
//
// Example 1:
//
// Input
// ["MyStack", "push", "push", "top", "pop", "empty"]
// [[], [1], [2], [], [], []]
// Output
// [null, null, null, 2, 2, false]
//
// Explanation
// MyStack myStack = new MyStack();
// myStack.push(1);
// myStack.push(2);
// myStack.top(); // return 2
// myStack.pop(); // return 2
// myStack.empty(); // return False
//
// Constraints:
//
// - 1 <= x <= 9
// - At most 100 calls will be made to push, pop, top, and empty.
// - All the calls to pop and top are valid.
//
// Follow-up: Can you implement the stack such that each operation is amortized O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.
//

use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut temp = VecDeque::new();
        while self.queue.len() > 1 {
            temp.push_back(self.queue.pop_front().unwrap());
        }
        let result = self.queue.pop_front().unwrap();
        self.queue = temp;
        result
    }

    fn top(&self) -> i32 {
        *self.queue.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[test]
fn test() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}
