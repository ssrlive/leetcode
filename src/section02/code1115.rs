#![allow(dead_code)]

// 1115. Print FooBar Alternately
// https://leetcode.com/problems/print-foobar-alternately/
// https://leetcode.cn/problems/print-foobar-alternately/
//
// Suppose you are given the following code:
//
// class FooBar {
//   public void foo() {
//     for (int i = 0; i < n; i++) {
//       print("foo");
//     }
//   }
//
//   public void bar() {
//     for (int i = 0; i < n; i++) {
//       print("bar");
//     }
//   }
// }
//
// The same instance of FooBar will be passed to two different threads:
//
// thread A will call foo(), while
// thread B will call bar().
// Modify the given program to output "foobar" n times.
//
// Example 1:
//
// Input: n = 1
// Output: "foobar"
// Explanation: There are two threads being fired asynchronously. One of them calls foo(), while the other calls bar().
// "foobar" is being output 1 time.
//
// Example 2:
//
// Input: n = 2
// Output: "foobarfoobar"
// Explanation: "foobar" is being output 2 times.
//
// Constraints:
//
// - 1 <= n <= 1000
//

/*
// cpp solution
class FooBar {
private:
    int n;

    mutex lock;
    condition_variable foo_cv;
    condition_variable bar_cv;
    bool print_foo = true;

public:
    FooBar(int n) {
        this->n = n;
        foo_cv.notify_all();
    }

    void foo(function<void()> printFoo) {
        for (int i = 0; i < n; i++) {
            {
                unique_lock<mutex> ul(lock);
                foo_cv.wait(ul, [&](){return print_foo == true;});
            }
            printFoo();
            {
                unique_lock<mutex> ul(lock);
                print_foo = false;
            }
            bar_cv.notify_one();
        }
    }

    void bar(function<void()> printBar) {
        for (int i = 0; i < n; i++) {
            {
                unique_lock<mutex> ul(lock);
                bar_cv.wait(ul, [&](){return print_foo == false;});
            }
            printBar();
            {
                unique_lock<mutex> ul(lock);
                print_foo = true;
            }
            foo_cv.notify_one();
        }
    }
};
*/

use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, Clone)]
struct FooBar {
    n: i32,
    lock: Arc<Mutex<()>>,
    foo_cv: Arc<Condvar>,
    bar_cv: Arc<Condvar>,
    is_print_foo: Arc<Mutex<bool>>,
}

impl FooBar {
    fn new(n: i32) -> Self {
        Self {
            n,
            lock: Arc::new(Mutex::new(())),
            foo_cv: Arc::new(Condvar::new()),
            bar_cv: Arc::new(Condvar::new()),
            is_print_foo: Arc::new(Mutex::new(true)),
        }
    }

    fn foo(&self, print_foo: impl Fn()) {
        for _i in 0..self.n {
            {
                let lock = self.lock.lock().unwrap();
                let is_print_foo = { *self.is_print_foo.lock().unwrap() };
                if !is_print_foo {
                    let _unused = self.foo_cv.wait(lock).unwrap();
                }
            }
            print_foo();
            {
                let _lock = self.lock.lock().unwrap();
                *self.is_print_foo.lock().unwrap() = false;
            }
            self.bar_cv.notify_one();
        }
    }

    fn bar(&self, print_bar: impl Fn()) {
        for _i in 0..self.n {
            {
                let lock = self.lock.lock().unwrap();
                let is_print_foo = { *self.is_print_foo.lock().unwrap() };
                if is_print_foo {
                    let _unused = self.bar_cv.wait(lock).unwrap();
                }
            }
            print_bar();
            {
                let _lock = self.lock.lock().unwrap();
                *self.is_print_foo.lock().unwrap() = true;
            }
            self.foo_cv.notify_one();
        }
    }
}

#[test]
fn test() {
    use std::thread;
    let foo_bar = FooBar::new(3);
    let foo_bar1 = foo_bar.clone();
    let foo_bar2 = foo_bar.clone();
    let foo3 = thread::spawn(move || {
        foo_bar1.foo(|| {
            print!("foo");
        });
    });
    let bar = thread::spawn(move || {
        foo_bar2.bar(|| {
            print!("bar");
        });
    });
    foo3.join().unwrap();
    bar.join().unwrap();
    println!();
}
