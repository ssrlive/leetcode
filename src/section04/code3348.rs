#![allow(dead_code)]

// 3348. Smallest Divisible Digit Product II
// https://leetcode.com/problems/smallest-divisible-digit-product-ii/
// https://leetcode.cn/problems/smallest-divisible-digit-product-ii/
//
// Hard
//
// You are given a string num which represents a positive integer, and an integer t.
//
// A number is called zero-free if none of its digits are 0.
//
// Return a string representing the smallest zero-free number greater than or equal to num
// such that the product of its digits is divisible by t. If no such number exists, return "-1".
//
// Example 1:
//
// Input: num = "1234", t = 256
//
// Output: "1488"
//
// Explanation:
//
// The smallest zero-free number that is greater than 1234 and has the product of its digits
// divisible by 256 is 1488, with the product of its digits equal to 256.
//
// Example 2:
//
// Input: num = "12355", t = 50
//
// Output: "12355"
//
// Explanation:
//
// 12355 is already zero-free and has the product of its digits divisible by 50, with the product of its digits equal to 150.
//
// Example 3:
//
// Input: num = "11111", t = 26
//
// Output: "-1"
//
// Explanation:
//
// No number greater than 11111 has the product of its digits divisible by 26.
//
// Constraints:
//
//     2 <= num.length <= 2 * 10^5
//     num consists only of digits in the range ['0', '9'].
//     num does not contain leading zeros.
//     1 <= t <= 10^14
//

struct Solution;

impl Solution {
    pub fn smallest_number(num0: String, t: i64) -> String {
        let num = num0.as_bytes();
        let mut vec = Self::get(t);
        if vec.is_empty() {
            return "-1".to_string();
        }
        let mut prefix = vec![0; 4];
        for &c in num {
            let temp = Self::get(c as i64 - b'0' as i64);
            Self::add(&mut prefix, &temp);
        }
        let mut zero_index = -1;
        for (i, &c) in num.iter().enumerate() {
            if c == b'0' {
                zero_index = i as i32;
                break;
            }
        }
        if Self::ok(&prefix, &vec) && zero_index == -1 {
            return num0;
        }
        let mut post_len = 0;
        for i in (0..num.len()).rev() {
            let d = num[i] as i64 - b'0' as i64;
            let temp = Self::get(d);
            Self::remove(&mut prefix, &temp);
            if zero_index != -1 && i as i32 > zero_index {
                post_len += 1;
                continue;
            }
            for j in (d + 1)..=9 {
                let temp = Self::get(j);
                Self::add(&mut prefix, &temp);
                Self::remove(&mut vec, &prefix);
                let mut t = Self::construct(&vec);
                let mut cnt = 0;
                for &t_k in t.iter().take(10).skip(2) {
                    cnt += t_k;
                }
                if cnt <= post_len {
                    let temp = Self::add1(&prefix, &t);
                    if Self::ok(&temp, &vec) {
                        let mut ans = num[..i].iter().map(|&c| c as char).collect::<String>();
                        ans.push_str(&j.to_string());
                        while cnt != post_len {
                            cnt += 1;
                            ans.push('1');
                        }
                        for (k, t_k) in t.iter_mut().enumerate().take(10).skip(1) {
                            while *t_k > 0 {
                                ans.push((k as u8 + b'0') as char);
                                *t_k -= 1;
                            }
                        }
                        return ans;
                    }
                }
                Self::add(&mut vec, &prefix);
                Self::remove(&mut prefix, &temp);
            }
            post_len += 1;
        }
        let mut ans = String::new();
        let mut t = Self::construct(&vec);
        for (i, t_i) in t.iter_mut().enumerate().take(10).skip(1) {
            while *t_i > 0 {
                ans.push((i as u8 + b'0') as char);
                *t_i -= 1;
            }
        }
        let mut one = String::new();
        while one.len() + ans.len() <= num.len() {
            one.push('1');
        }
        one + &ans
    }

    fn construct(vec: &[i32]) -> Vec<i32> {
        let mut ans = vec![0; 10];
        if vec[2] > 0 {
            ans[5] = vec[2];
        }
        if vec[3] > 0 {
            ans[7] = vec[3];
        }
        let mut two = vec[0];
        let mut three = vec[1];
        two = two.max(0);
        three = three.max(0);
        while two >= 3 {
            two -= 3;
            ans[8] += 1;
        }
        while three >= 2 {
            three -= 2;
            ans[9] += 1;
        }
        if two == 1 && three == 0 {
            ans[2] += 1;
        } else if two == 2 && three == 1 {
            ans[2] += 1;
            ans[6] += 1;
        } else if two == 1 && three == 1 {
            ans[6] += 1;
        } else if two == 2 && three == 0 {
            ans[4] += 1;
        } else if two == 0 && three == 1 {
            ans[3] += 1;
        }
        ans
    }

    fn ok(vec1: &[i32], vec2: &[i32]) -> bool {
        for i in 0..4 {
            if vec1[i] < vec2[i] {
                return false;
            }
        }
        true
    }

    fn get(mut n: i64) -> Vec<i32> {
        let mut vec = vec![0; 4];
        if n == 0 {
            return vec;
        }
        while n % 2 == 0 {
            n /= 2;
            vec[0] += 1;
        }
        while n % 3 == 0 {
            n /= 3;
            vec[1] += 1;
        }
        while n % 5 == 0 {
            n /= 5;
            vec[2] += 1;
        }
        while n % 7 == 0 {
            n /= 7;
            vec[3] += 1;
        }
        if n != 1 {
            return vec![];
        }
        vec
    }

    fn add(vec1: &mut [i32], vec2: &[i32]) {
        for i in 0..4 {
            vec1[i] += vec2[i];
        }
    }

    fn add1(vec1: &[i32], vec10: &[i32]) -> Vec<i32> {
        let mut temp = vec1.to_vec();
        temp[0] += vec10[2] + vec10[6];
        temp[0] += 2 * vec10[4];
        temp[0] += 3 * vec10[8];

        temp[1] += vec10[3];
        temp[1] += vec10[6];
        temp[1] += 2 * vec10[9];

        temp[2] += vec10[5];
        temp[3] += vec10[7];
        temp
    }

    fn remove(vec1: &mut [i32], vec2: &[i32]) {
        for i in 0..4 {
            vec1[i] -= vec2[i];
        }
    }
}

#[test]
fn test() {
    let num = "1234".to_string();
    let t = 256;
    let output = "1488".to_string();
    assert_eq!(Solution::smallest_number(num, t), output);

    let num = "12355".to_string();
    let t = 50;
    let output = "12355".to_string();
    assert_eq!(Solution::smallest_number(num, t), output);

    let num = "11111".to_string();
    let t = 26;
    let output = "-1".to_string();
    assert_eq!(Solution::smallest_number(num, t), output);
}
