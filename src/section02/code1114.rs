#![allow(dead_code)]

// 1114. Print in Order
// https://leetcode.com/problems/print-in-order/
// https://leetcode.cn/problems/print-in-order/
//
// Suppose we have a class:
//
// public class Foo {
//   public void first() { print("first"); }
//   public void second() { print("second"); }
//   public void third() { print("third"); }
// }
//
// The same instance of Foo will be passed to three different threads. Thread A will call first(), thread B will call second(), and thread C will
// call third(). Design a mechanism and modify the program to ensure that second() is executed after first(), and third() is executed after second().
//
// Note:
//
// We do not know how the threads will be scheduled in the operating system, even though the numbers in the input seem to imply the ordering.
// The input format you see is mainly to ensure our tests' comprehensiveness.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: "firstsecondthird"
// Explanation: There are three threads being fired asynchronously. The input [1,2,3] means thread A calls first(),
// thread B calls second(), and thread C calls third(). "firstsecondthird" is the correct output.
//
// Example 2:
//
// Input: nums = [1,3,2]
// Output: "firstsecondthird"
// Explanation: The input [1,3,2] means thread A calls first(), thread B calls third(), and thread C calls second(). "firstsecondthird" is the correct output.
//
// Constraints:
//
// - nums is a permutation of [1, 2, 3].
//

use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

#[derive(Debug, Clone)]
struct Foo {
    count: Arc<Mutex<i32>>,
    cond: Arc<Condvar>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            count: Arc::new(Mutex::new(1)),
            cond: Arc::new(Condvar::new()),
        }
    }

    fn first(&self, print_first: impl FnOnce() + Send + 'static) {
        let count = self.count.clone();
        let cond = self.cond.clone();
        thread::spawn(move || {
            print_first();
            *count.lock().unwrap() = 2;
            cond.notify_all();
        });
    }

    fn second(&self, print_second: impl FnOnce() + Send + 'static) {
        let count = self.count.clone();
        let cond = self.cond.clone();
        thread::spawn(move || {
            let mut count = count.lock().unwrap();
            while *count != 2 {
                count = cond.wait(count).unwrap();
            }
            print_second();
            *count = 3;
            cond.notify_all();
        });
    }

    fn third(&self, print_third: impl FnOnce() + Send + 'static) {
        let count = self.count.clone();
        let cond = self.cond.clone();
        thread::spawn(move || {
            let mut count = count.lock().unwrap();
            while *count != 3 {
                count = cond.wait(count).unwrap();
            }
            print_third();
        });
    }
}

#[test]
fn test() {
    let foo = Foo::new();
    let foo1 = foo.clone();
    let foo2 = foo.clone();
    let foo3 = foo.clone();
    let t1 = thread::spawn(move || {
        foo1.first(|| {
            println!("first");
        });
    });
    let t2 = thread::spawn(move || {
        foo2.second(|| {
            println!("second");
        });
    });
    let t3 = thread::spawn(move || {
        foo3.third(|| {
            println!("third");
        });
    });
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    // std::thread::sleep(std::time::Duration::from_secs(1));
}
