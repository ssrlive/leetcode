#![allow(dead_code)]

/*

// 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
// https://leetcode.cn/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
//
// Medium
//
// You are given a 0-indexed array nums consisiting of positive integers. You can do the following operation on the array any number of times:

    Select an index i such that 0 <= i < n - 1 and replace either of nums[i] or nums[i+1] with their gcd value.

Return the minimum number of operations to make all elements of nums equal to 1. If it is impossible, return -1.

The gcd of two integers is the greatest common divisor of the two integers.

Example 1:

Input: nums = [2,6,3,4]
Output: 4
Explanation: We can do the following operations:
- Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
- Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
- Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
- Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].

Example 2:

Input: nums = [2,10,6,14]
Output: -1
Explanation: It can be shown that it is impossible to make all the elements equal to 1.

Constraints:

    2 <= nums.length <= 50
    1 <= nums[i] <= 10^6

Follow-up:

The O(n) time complexity solution works, but could you find an O(1) constant time complexity solution?
*/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut one = 0;
        for &num in nums.iter() {
            if num == 1 {
                one += 1;
            }
        }
        if one > 0 {
            return (n - one) as i32;
        }
        let mut la = vec![];
        let mut ra = vec![];
        let mut lg = vec![];
        let mut rg = vec![];

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        fn good(la: &Vec<i32>, ra: &Vec<i32>, lg: &Vec<i32>, rg: &Vec<i32>) -> bool {
            let mut g = 0;
            if !la.is_empty() {
                g = gcd(g, lg[lg.len() - 1]);
            }
            if !ra.is_empty() {
                g = gcd(g, rg[rg.len() - 1]);
            }
            g == 1
        }

        fn add(ra: &mut Vec<i32>, rg: &mut Vec<i32>, x: i32) {
            ra.push(x);
            rg.push(gcd(if rg.is_empty() { 0 } else { rg[rg.len() - 1] }, x));
        }

        fn remove(la: &mut Vec<i32>, ra: &mut Vec<i32>, lg: &mut Vec<i32>, rg: &mut Vec<i32>) {
            if la.is_empty() {
                while !ra.is_empty() {
                    let x = ra[ra.len() - 1];
                    la.push(x);
                    ra.pop();
                    rg.pop();
                    lg.push(gcd(if lg.is_empty() { 0 } else { lg[lg.len() - 1] }, x));
                }
            }
            la.pop();
            lg.pop();
        }

        let mut ans = n;
        let (mut l, mut r) = (0, 0);
        while r < n {
            add(&mut ra, &mut rg, nums[r]);
            while l < n && good(&la, &ra, &lg, &rg) {
                ans = ans.min(r - l);
                remove(&mut la, &mut ra, &mut lg, &mut rg);
                l += 1;
            }
            r += 1;
        }
        if ans == n {
            return -1;
        }
        (n + ans - 1) as i32
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![2, 6, 3, 4], 4),
        (vec![2, 10, 6, 14], -1),
    ];
    for (nums, expect) in cases {
        assert_eq!(Solution::min_operations(nums.clone()), expect);
    }
}
