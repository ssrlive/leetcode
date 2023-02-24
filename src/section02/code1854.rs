#![allow(dead_code)]

/*

// 1854. Maximum Population Year
// https://leetcode.com/problems/maximum-population-year/
// https://leetcode.cn/problems/maximum-population-year/
//
// Easy
//
// You are given a 2D integer array logs where each logs[i] = [birthi, deathi] indicates the birth and death years of the ith person.

The population of some year x is the number of people alive during that year. The ith person is counted in year x's population if x is in the inclusive range [birthi, deathi - 1]. Note that the person is not counted in the year that they die.

Return the earliest year with the maximum population.

Example 1:

Input: logs = [[1993,1999],[2000,2010]]
Output: 1993
Explanation: The maximum population is 1, and 1993 is the earliest year with this population.

Example 2:

Input: logs = [[1950,1961],[1960,1971],[1970,1981]]
Output: 1960
Explanation:
The maximum population is 2, and it had happened in years 1960 and 1970.
The earlier year between them is 1960.

Constraints:

    1 <= logs.length <= 100
    1950 <= birthi < deathi <= 2050
*/

struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let (mut res_year, mut max_pop) = (0, 0);
        let mut year_pop_arr = [0; 101];
        for years in &logs {
            for year in years[0]..years[1] {
                let ind_year = (year - 1950) as usize;
                let new_pop = year_pop_arr[ind_year] + 1;
                year_pop_arr[ind_year] = new_pop;
                if new_pop > max_pop || new_pop == max_pop && res_year > year {
                    res_year = year;
                    max_pop = new_pop;
                }
            }
        }
        res_year
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![1993, 1999], vec![2000, 2010]], 1993),
        (vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]], 1960),
    ];
    for (logs, expected) in cases {
        assert_eq!(Solution::maximum_population(logs), expected);
    }
}
