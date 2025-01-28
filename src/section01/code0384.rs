#![allow(dead_code)]

// 384. Shuffling an Array
// https://leetcode.com/problems/shuffle-an-array/
// https://leetcode.cn/problems/shuffle-an-array/
//
// Given an integer array nums, design an algorithm to randomly shuffle the array.
// All permutations of the array should be equally likely as a result of the shuffling.
//
// Implement the Solution class:
//
// - Solution(int[] nums) Initializes the object with the integer array nums.
// - int[] reset() Resets the array to its original configuration and returns it.
// - int[] shuffle() Returns a random shuffling of the array.
//
// Example 1:
//
// Input
// ["Solution", "shuffle", "reset", "shuffle"]
// [[[1, 2, 3]], [], [], []]
// Output
// [null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
//
// Explanation
// Solution solution = new Solution([1, 2, 3]);
// solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
//                        // Any permutation of [1,2,3] must be equally likely to be returned.
//                        // Example: return [3, 1, 2]
// solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
// solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]
//
// Constraints:
//
// - 1 <= nums.length <= 50
// - -10^6 <= nums[i] <= 10^6
// - All the elements of nums are unique.
// - At most 104 calls in total will be made to reset and shuffle.
//

use rand::distr::{Distribution, Uniform};
use rand::prelude::*;

struct Solution {
    origin: Vec<i32>,
    shuffled: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            origin: nums.clone(),
            shuffled: nums,
            rng: rand::rng(),
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.shuffled.clone_from(&self.origin);
        self.origin.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let n = self.shuffled.len();
        for i in 0..(n - 1) {
            let uniform = Uniform::try_from(i..n).unwrap();
            let j = uniform.sample(&mut self.rng);
            self.shuffled.swap(i, j);
        }
        self.shuffled.clone()
    }
}

#[test]
fn test() {
    let mut solution = Solution::new(vec![1, 2, 3]);
    let mut reset = solution.reset();
    reset.sort();
    assert_eq!(reset, vec![1, 2, 3]);
    let mut shuffle = solution.shuffle();
    shuffle.sort();
    assert_eq!(shuffle, vec![1, 2, 3]);
}
