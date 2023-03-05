#![allow(dead_code)]

/*

// 2197. Replace Non-Coprime Numbers in Array
// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/
// https://leetcode.cn/problems/replace-non-coprime-numbers-in-array/
//
// Hard
//
// You are given an array of integers nums. Perform the following steps:

    Find any two adjacent numbers in nums that are non-coprime.
    If no such numbers are found, stop the process.
    Otherwise, delete the two numbers and replace them with their LCM (Least Common Multiple).
    Repeat this process as long as you keep finding two adjacent non-coprime numbers.

Return the final modified array. It can be shown that replacing adjacent non-coprime numbers in any arbitrary order will lead to the same result.

The test cases are generated such that the values in the final array are less than or equal to 108.

Two values x and y are non-coprime if GCD(x, y) > 1 where GCD(x, y) is the Greatest Common Divisor of x and y.

Example 1:

Input: nums = [6,4,3,2,7,6,2]
Output: [12,7,6]
Explanation:
- (6, 4) are non-coprime with LCM(6, 4) = 12. Now, nums = [12,3,2,7,6,2].
- (12, 3) are non-coprime with LCM(12, 3) = 12. Now, nums = [12,2,7,6,2].
- (12, 2) are non-coprime with LCM(12, 2) = 12. Now, nums = [12,7,6,2].
- (6, 2) are non-coprime with LCM(6, 2) = 6. Now, nums = [12,7,6].
There are no more adjacent non-coprime numbers in nums.
Thus, the final modified array is [12,7,6].
Note that there are other ways to obtain the same resultant array.

Example 2:

Input: nums = [2,2,1,1,3,3,3]
Output: [2,1,1,3]
Explanation:
- (3, 3) are non-coprime with LCM(3, 3) = 3. Now, nums = [2,2,1,1,3,3].
- (3, 3) are non-coprime with LCM(3, 3) = 3. Now, nums = [2,2,1,1,3].
- (2, 2) are non-coprime with LCM(2, 2) = 2. Now, nums = [2,1,1,3].
There are no more adjacent non-coprime numbers in nums.
Thus, the final modified array is [2,1,1,3].
Note that there are other ways to obtain the same resultant array.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^5
    The test cases are generated such that the values in the final array are less than or equal to 10^8.
*/

struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(a: i32, b: i32) -> i32 {
            match b {
                0 => a,
                _ => gcd(b, a % b),
            }
        }

        fn lcm(a: i32, b: i32, d: i32) -> i32 {
            let a = a as i64;
            let b = b as i64;
            let d = d as i64;
            let ans = (a * b) / d;
            ans as i32
        }

        let mut res = vec![];
        for v in nums.iter() {
            let n = res.len();
            if n == 0 {
                res.push(*v);
                continue;
            }
            let mut m = *v;
            loop {
                let n = res.len() as i32;
                if n <= 0 {
                    break;
                }

                let tail = res.pop().unwrap();
                let d = gcd(m, tail);
                if d == 1 {
                    res.push(tail);
                    break;
                }
                m = lcm(m, tail, d);
            }
            res.push(m);
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![6, 4, 3, 2, 7, 6, 2], vec![12, 7, 6]),
        (vec![2, 2, 1, 1, 3, 3, 3], vec![2, 1, 1, 3]),
    ];
    for (nums, expect) in cases {
        let res = Solution::replace_non_coprimes(nums);
        assert_eq!(res, expect);
    }
}
