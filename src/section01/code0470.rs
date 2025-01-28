#![allow(dead_code)]

// 470. Implement Rand10() Using Rand7()
// https://leetcode.com/problems/implement-rand10-using-rand7/
// https://leetcode.cn/problems/implement-rand10-using-rand7/
//
// Given the API rand7() that generates a uniform random integer in the range [1, 7], write a function rand10()
// that generates a uniform random integer in the range [1, 10]. You can only call the API rand7(), and you shouldn't
// call any other API. Please do not use a language's built-in random API.
//
// Each test case will have one internal argument n, the number of times that your implemented function rand10()
// will be called while testing. Note that this is not an argument passed to rand10().
//
// Example 1:
//
// Input: n = 1
// Output: [2]
//
// Example 2:
//
// Input: n = 2
// Output: [2,8]
//
// Example 3:
//
// Input: n = 3
// Output: [3,8,10]
//
// Constraints:
//
// - 1 <= n <= 10^5
//
// Follow up:
//
// What is the expected value for the number of calls to rand7() function?
// Could you minimize the number of calls to rand7()?
//

struct Solution;

impl Solution {
    pub fn rand10() -> i32 {
        let mut result = 0;
        for _ in 0..10 {
            result += rand7();
        }
        result % 10 + 1
    }
}

fn rand7() -> i32 {
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random_range(1..8)
}

#[test]
fn test() {
    let mut count = vec![0; 10];
    for _ in 0..100000 {
        let n = Solution::rand10();
        count[n as usize - 1] += 1;
    }
    println!("{:?}", count);
}
