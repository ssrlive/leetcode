#![allow(dead_code)]

// 519. Random Flip Matrix
// https://leetcode.com/problems/random-flip-matrix/
//
// There is an m x n binary grid matrix with all the values set 0 initially. Design an algorithm to randomly pick an index (i, j) where matrix[i][j] == 0 and flips it to 1. All the indices (i, j) where matrix[i][j] == 0 should be equally likely to be returned.
//
// Optimize your algorithm to minimize the number of calls made to the built-in random function of your language and optimize the time and space complexity.
//
// Implement the Solution class:
//
// Solution(int m, int n) Initializes the object with the size of the binary matrix m and n.
// int[] flip() Returns a random index [i, j] of the matrix where matrix[i][j] == 0 and flips it to 1.
// void reset() Resets all the values of the matrix to be 0.
//
// Example 1:
//
// Input
// ["Solution", "flip", "flip", "flip", "reset", "flip"]
// [[3, 1], [], [], [], [], []]
// Output
// [null, [1, 0], [2, 0], [0, 0], null, [2, 0]]
//
// Explanation
// Solution solution = new Solution(3, 1);
// solution.flip();  // return [1, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
// solution.flip();  // return [2, 0], Since [1,0] was returned, [2,0] and [0,0]
// solution.flip();  // return [0, 0], Based on the previously returned indices, only [0,0] can be returned.
// solution.reset(); // All the values are reset to 0 and can be returned.
// solution.flip();  // return [2, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
//
// Constraints:
//
// - 1 <= m, n <= 104
// - There will be at least one free cell for each call to flip.
// - At most 1000 calls will be made to flip and reset.
//

struct Solution {
    m: i32,
    n: i32,
    total: i32,
    used: std::collections::HashSet<i32>,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            total: m * n,
            used: std::collections::HashSet::new(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0 .. self.total);
        while self.used.contains(&index) {
            index = rng.gen_range(0 .. self.total);
        }
        self.used.insert(index);
        vec![index / self.n, index % self.n]
    }

    fn reset(&mut self) {
        self.used.clear();
    }
}

#[test]
fn test() {
    let mut solution = Solution::new(3, 1);
    println!("{:?}", solution.flip());
    println!("{:?}", solution.flip());
    println!("{:?}", solution.flip());
    // assert_eq!(solution.flip(), vec![1, 0]);
    // assert_eq!(solution.flip(), vec![2, 0]);
    // assert_eq!(solution.flip(), vec![0, 0]);
    solution.reset();
    // assert_eq!(solution.flip(), vec![2, 0]);
    println!("{:?}", solution.flip());
}
