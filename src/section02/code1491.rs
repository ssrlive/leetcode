#![allow(dead_code)]

/*

// 1491. Average Salary Excluding the Minimum and Maximum Salary
// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
// https://leetcode.cn/problems/average-salary-excluding-the-minimum-and-maximum-salary/
//
// Easy
//
// You are given an array of unique integers salary where salary[i] is the salary of the ith employee.

Return the average salary of employees excluding the minimum and maximum salary. Answers within 10-5 of the actual answer will be accepted.

Example 1:

Input: salary = [4000,3000,1000,2000]
Output: 2500.00000
Explanation: Minimum salary and maximum salary are 1000 and 4000 respectively.
Average salary excluding minimum and maximum salary is (2000+3000) / 2 = 2500

Example 2:

Input: salary = [1000,2000,3000]
Output: 2000.00000
Explanation: Minimum salary and maximum salary are 1000 and 3000 respectively.
Average salary excluding minimum and maximum salary is (2000) / 1 = 2000

Constraints:

    3 <= salary.length <= 100
    1000 <= salary[i] <= 10^6
    All the integers of salary are unique.
*/

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = salary[0];
        let mut max = salary[0];
        let mut sum = 0;
        for &s in salary.iter() {
            if s < min {
                min = s;
            }
            if s > max {
                max = s;
            }
            sum += s;
        }
        (sum - min - max) as f64 / (salary.len() - 2) as f64
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4000, 3000, 1000, 2000], 2500.00000),
        (vec![1000, 2000, 3000], 2000.00000),
    ];
    for (salary, expected) in cases {
        assert_eq!(Solution::average(salary), expected);
    }
}
