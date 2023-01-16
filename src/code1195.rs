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

/*
// cpp solution
class FizzBuzz {
public:
    explicit FizzBuzz(int n): n(n), i(1), mutex(), condition() {}

    void fizz(std::function<void()> printFizz) {
        auto predicate = [](int v) { return (v % 3 == 0) && (v % 5 != 0); };
        auto printer = [printFizz](int) { printFizz(); };
        run(printer, predicate);
    }

    void buzz(std::function<void()> printBuzz) {
        auto predicate = [](int v) { return (v % 3 != 0) && (v % 5 == 0); };
        auto printer = [printBuzz](int) { printBuzz(); };
        run(printer, predicate);
    }

    void fizzbuzz(std::function<void()> printFizzBuzz) {
        auto predicate = [](int v) { return (v % 3 == 0) && (v % 5 == 0); };
        auto printer = [printFizzBuzz](int) { printFizzBuzz(); };
        run(printer, predicate);
    }

    void number(std::function<void(int)> printNumber) {
        auto predicate = [](int v) { return (v % 3 != 0) && (v % 5 != 0); };
        run(printNumber, predicate);
    }

private:
    int n;
    int i;
    std::mutex mutex;
    std::condition_variable condition;

    template <typename Printer, typename Predicate>
    void run(Printer printer, Predicate predicate) {
        for (;;) {
            int v = 0;
            {
                std::unique_lock<std::mutex> lock(mutex);
                condition.wait(lock, [this, predicate]() {
                    return (i > n) || predicate(i);
                });
                if (i > n) {
                    break;
                }
                v = i;
            }
            printer(v);
            {
                std::unique_lock<std::mutex> lock(mutex);
                i++;
            }
            condition.notify_all();
        }
    }
};

// rust solution can't pass. I don't know how to implement it in rust.
*/

struct FizzBuzz {}

impl FizzBuzz {}

#[test]
fn test() {}
