#![allow(dead_code)]

// 600. Non-negative Integers without Consecutive Ones
// https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/
//
// Given a positive integer n, return the number of the integers in the range [0, n] whose binary representations do not contain consecutive ones.
//
// Example 1:
//
// Input: n = 5
// Output: 5
// Explanation:
// Here are the non-negative integers <= 5 with their corresponding binary representations:
// 0 : 0
// 1 : 1
// 2 : 10
// 3 : 11
// 4 : 100
// 5 : 101
// Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.
//
// Example 2:
//
// Input: n = 1
// Output: 2
//
// Example 3:
//
// Input: n = 2
// Output: 3
//
// Constraints:
//
// - 1 <= n <= 10^9
//

struct Solution;

impl Solution {
    /*
    // cpp solution
    int findIntegers(int num) {
         if(num == 0) {
            return 1;
        }
         if(num == 1) {
            return 2;
        }

         //STEP 1
         int digit = 0;
         while(num >= 1 << digit) {
            digit++;
         }
         int *digitsOfNum = new int[digit];
         int *zero = new int[digit];
         int *one = new int[digit];

         //not include 0 in DP
         zero[0] = 0;
         zero[1] = 1;
         one[0] = 1;
         one[1] = 0;

         int sum = 1; //include 0

         for(int i = 0; i < digit; i++) {
            if(i > 1) {
                zero[i] = zero[i - 1] + zero[i - 2];
                one[i] = one[i - 1] + one[i - 2];
            }
            if(i < digit - 1) {
                sum += zero[i] + one[i];
            }
            digitsOfNum[digit - 1 - i] = num % 2;
            num >>= 1;
         }

         // STEP 2

         int i = 0;
         bool isValid = true;

         while(true) {
             if(i + 1 == digit) {
                break;
             }
             if(isValid) {
                 if(digitsOfNum[i] != digitsOfNum[i + 1]) {
                     zero[i + 1] = zero[i] + one[i];
                     one[i + 1] = zero[i];
                     i++;
                 } else {
                     if(digitsOfNum[i] == 1) {
                         isValid = false;
                     }else{
                         zero[i + 1] = zero[i] + one[i];
                         one[i + 1] = zero[i] - 1;
                         i++;
                     }
                 }
             }else{
                 zero[i + 1] = zero[i] + one[i];
                 one[i + 1] = zero[i];
                 i++;
             }
         }

         return sum + zero[digit - 1] + one[digit - 1];
    }
    */

    // rust solution
    pub fn find_integers(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        if n == 1 {
            return 2;
        }

        let mut num = n;
        let mut digit = 0;
        while num >= 1 << digit {
            digit += 1;
        }
        let mut digits_of_num = vec![0; digit as usize];
        let mut zero = vec![0; digit as usize];
        let mut one = vec![0; digit as usize];

        zero[0] = 0;
        zero[1] = 1;
        one[0] = 1;
        one[1] = 0;

        let mut sum = 1;

        for i in 0..digit {
            if i > 1 {
                zero[i as usize] = zero[(i - 1) as usize] + zero[(i - 2) as usize];
                one[i as usize] = one[(i - 1) as usize] + one[(i - 2) as usize];
            }
            if i < digit - 1 {
                sum += zero[i as usize] + one[i as usize];
            }
            digits_of_num[(digit - 1 - i) as usize] = num % 2;
            num >>= 1;
        }

        let mut i = 0;
        let mut is_valid = true;

        while i + 1 < digit {
            if is_valid {
                if digits_of_num[i as usize] != digits_of_num[(i + 1) as usize] {
                    zero[(i + 1) as usize] = zero[i as usize] + one[i as usize];
                    one[(i + 1) as usize] = zero[i as usize];
                    i += 1;
                } else if digits_of_num[i as usize] == 1 {
                    is_valid = false;
                } else {
                    zero[(i + 1) as usize] = zero[i as usize] + one[i as usize];
                    one[(i + 1) as usize] = zero[i as usize] - 1;
                    i += 1;
                }
            } else {
                zero[(i + 1) as usize] = zero[i as usize] + one[i as usize];
                one[(i + 1) as usize] = zero[i as usize];
                i += 1;
            }
        }

        sum + zero[(digit - 1) as usize] + one[(digit - 1) as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_integers(5), 5);
    assert_eq!(Solution::find_integers(1), 2);
    assert_eq!(Solution::find_integers(2), 3);
}
