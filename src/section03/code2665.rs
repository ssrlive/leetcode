#![allow(dead_code)]

// 2665. Counter II
// https://leetcode.com/problems/counter-ii/
// https://leetcode.cn/problems/counter-ii/
//
// Easy
//
// Write a function createCounter. It should accept an initial integer init. It should return an object with three functions.
// The three functions are:
//
//    increment() increases the current value by 1 and then returns it.
//    decrement() reduces the current value by 1 and then returns it.
//    reset() sets the current value to init and then returns it.
//
// Example 1:
//
// Input: init = 5, calls = ["increment","reset","decrement"]
// Output: [6,5,4]
// Explanation:
// const counter = createCounter(5);
// counter.increment(); // 6
// counter.reset(); // 5
// counter.decrement(); // 4
//
// Example 2:
//
// Input: init = 0, calls = ["increment","increment","decrement","reset","reset"]
// Output: [1,2,1,0,0]
// Explanation:
// const counter = createCounter(0);
// counter.increment(); // 1
// counter.increment(); // 2
// counter.decrement(); // 1
// counter.reset(); // 0
// counter.reset(); // 0
//
// Constraints:
//
//     -1000 <= init <= 1000
//     0 <= calls.length <= 1000
//     calls[i] is one of "increment", "decrement" and "reset"
//
//

struct Counter {
    num: i32,
}

impl Counter {
    fn new(init: i32) -> Self {
        Self { num: init }
    }

    fn increment(&mut self) -> i32 {
        self.num += 1;
        self.num
    }

    fn decrement(&mut self) -> i32 {
        self.num -= 1;
        self.num
    }

    fn reset(&mut self, init: i32) -> i32 {
        self.num = init;
        self.num
    }
}

#[test]
fn test() {
    let mut counter = Counter::new(5);
    assert_eq!(counter.increment(), 6);
    assert_eq!(counter.reset(5), 5);
    assert_eq!(counter.decrement(), 4);

    let mut counter = Counter::new(0);
    assert_eq!(counter.increment(), 1);
    assert_eq!(counter.increment(), 2);
    assert_eq!(counter.decrement(), 1);
    assert_eq!(counter.reset(0), 0);
    assert_eq!(counter.reset(0), 0);
}
