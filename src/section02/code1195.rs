#![allow(dead_code)]

// 1195. Fizz Buzz Multithreaded
// https://leetcode.com/problems/fizz-buzz-multithreaded/
// https://leetcode.cn/problems/fizz-buzz-multithreaded/
//
// You have the four functions:
//
// printFizz that prints the word "fizz" to the console,
// printBuzz that prints the word "buzz" to the console,
// printFizzBuzz that prints the word "fizzbuzz" to the console, and
// printNumber that prints a given integer to the console.
// You are given an instance of the class FizzBuzz that has four functions: fizz, buzz, fizzbuzz and number.
// The same instance of FizzBuzz will be passed to four different threads:
//
// Thread A: calls fizz() that should output the word "fizz".
// Thread B: calls buzz() that should output the word "buzz".
// Thread C: calls fizzbuzz() that should output the word "fizzbuzz".
// Thread D: calls number() that should only output the integers.
// Modify the given class to output the series [1, 2, "fizz", 4, "buzz", ...] where the ith token (1-indexed) of the series is:
//
// "fizzbuzz" if i is divisible by 3 and 5,
// "fizz" if i is divisible by 3 and not 5,
// "buzz" if i is divisible by 5 and not 3, or
// i if i is not divisible by 3 or 5.
// Implement the FizzBuzz class:
//
// FizzBuzz(int n) Initializes the object with the number n that represents the length of the sequence that should be printed.
// void fizz(printFizz) Calls printFizz to output "fizz".
// void buzz(printBuzz) Calls printBuzz to output "buzz".
// void fizzbuzz(printFizzBuzz) Calls printFizzBuzz to output "fizzbuzz".
// void number(printNumber) Calls printnumber to output the numbers.
//
// Example 1:
//
// Input: n = 15
// Output: [1,2,"fizz",4,"buzz","fizz",7,8,"fizz","buzz",11,"fizz",13,14,"fizzbuzz"]
//
// Example 2:
//
// Input: n = 5
// Output: [1,2,"fizz",4,"buzz"]
//
// Constraints:
//
// 1 <= n <= 50
//

use std::sync::{Arc, Barrier};

struct FizzBuzz {
    n: i32,
    barrier: Arc<Barrier>,
}

impl FizzBuzz {
    fn new(n: i32) -> Self {
        Self {
            n,
            barrier: Arc::new(Barrier::new(4)),
        }
    }

    fn fizz(&self, print_fizz: impl Fn()) {
        for i in 1..=self.n {
            if i % 3 == 0 && i % 5 != 0 {
                print_fizz();
            }
            self.barrier.wait();
        }
    }

    fn buzz(&self, print_buzz: impl Fn()) {
        for i in 1..=self.n {
            if i % 3 != 0 && i % 5 == 0 {
                print_buzz();
            }
            self.barrier.wait();
        }
    }

    fn fizzbuzz(&self, print_fizzbuzz: impl Fn()) {
        for i in 1..=self.n {
            if i % 3 == 0 && i % 5 == 0 {
                print_fizzbuzz();
            }
            self.barrier.wait();
        }
    }

    fn number(&self, print_number: impl Fn(i32)) {
        for i in 1..=self.n {
            if i % 3 != 0 && i % 5 != 0 {
                print_number(i);
            }
            self.barrier.wait();
        }
    }
}

#[test]
fn test() {
    use std::sync::Mutex;

    #[derive(Debug, PartialEq, Clone)]
    enum FizzBuzzValue {
        Fizz,
        Buzz,
        FizzBuzz,
        Number(i32),
    }

    fn test_fizz_buzz(n: i32) -> Vec<FizzBuzzValue> {
        let result0 = Arc::new(Mutex::new(Vec::new()));

        let fizz_buzz = Arc::new(FizzBuzz::new(n));

        let fizz = fizz_buzz.clone();
        let result = result0.clone();
        let t1 = std::thread::spawn(move || fizz.fizz(|| result.lock().unwrap().push(FizzBuzzValue::Fizz)));

        let buzz = fizz_buzz.clone();
        let result = result0.clone();
        let t2 = std::thread::spawn(move || buzz.buzz(|| result.lock().unwrap().push(FizzBuzzValue::Buzz)));

        let fizzbuzz = fizz_buzz.clone();
        let result = result0.clone();
        let t3 = std::thread::spawn(move || fizzbuzz.fizzbuzz(|| result.lock().unwrap().push(FizzBuzzValue::FizzBuzz)));

        let number = fizz_buzz;
        let result = result0.clone();
        let t4 = std::thread::spawn(move || number.number(|x| result.lock().unwrap().push(FizzBuzzValue::Number(x))));

        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        t4.join().unwrap();

        result0.lock().unwrap().clone()
    }

    assert_eq!(
        test_fizz_buzz(15),
        vec![
            FizzBuzzValue::Number(1),
            FizzBuzzValue::Number(2),
            FizzBuzzValue::Fizz,
            FizzBuzzValue::Number(4),
            FizzBuzzValue::Buzz,
            FizzBuzzValue::Fizz,
            FizzBuzzValue::Number(7),
            FizzBuzzValue::Number(8),
            FizzBuzzValue::Fizz,
            FizzBuzzValue::Buzz,
            FizzBuzzValue::Number(11),
            FizzBuzzValue::Fizz,
            FizzBuzzValue::Number(13),
            FizzBuzzValue::Number(14),
            FizzBuzzValue::FizzBuzz,
        ]
    );

    assert_eq!(
        test_fizz_buzz(5),
        vec![
            FizzBuzzValue::Number(1),
            FizzBuzzValue::Number(2),
            FizzBuzzValue::Fizz,
            FizzBuzzValue::Number(4),
            FizzBuzzValue::Buzz,
        ]
    );
}
