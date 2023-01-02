#![allow(dead_code)]

// 2521. Distinct Prime Factors of Product of Array
// https://leetcode.com/problems/distinct-prime-factors-of-product-of-array/
// https://leetcode.cn/problems/distinct-prime-factors-of-product-of-array/
//
// Given an array of positive integers nums, return the number of distinct prime factors in the product of the elements of nums.
//
// Note that:
//
// A number greater than 1 is called prime if it is divisible by only 1 and itself.
// An integer val1 is a factor of another integer val2 if val2 / val1 is an integer.
//
// Example 1:
//
// Input: nums = [2,4,3,7,10,6]
// Output: 4
// Explanation:
// The product of all the elements in nums is: 2 * 4 * 3 * 7 * 10 * 6 = 10080 = 25 * 32 * 5 * 7.
// There are 4 distinct prime factors so we return 4.
//
// Example 2:
//
// Input: nums = [2,4,8,16]
// Output: 1
// Explanation:
// The product of all the elements in nums is: 2 * 4 * 8 * 16 = 1024 = 2^10.
// There is 1 distinct prime factor so we return 1.
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - 2 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0;
        let mut mp = std::collections::HashMap::new();
        for num in &mut nums {
            let sq = (*num as f64).sqrt() as i32;
            for j in 2..=sq {
                if *num % j == 0 {
                    mp.insert(j, true);
                }
                while *num % j == 0 {
                    *num /= j;
                }
            }
            if *num > 1 {
                mp.insert(*num, true);
            }
        }
        for (k, _) in mp {
            println!("{}", k);
            count += 1;
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]), 4);
    assert_eq!(Solution::distinct_prime_factors(vec![2, 4, 8, 16]), 1);
}
