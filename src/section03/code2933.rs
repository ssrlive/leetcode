#![allow(dead_code)]

// 2933. High-Access Employees
// https://leetcode.com/problems/high-access-employees/
// https://leetcode.cn/problems/high-access-employees/
//
// Medium
//
// You are given a 2D 0-indexed array of strings, access_times, with size n. For each i where 0 <= i <= n - 1,
// access_times[i][0] represents the name of an employee, and access_times[i][1] represents the access time of that employee.
// All entries in access_times are within the same day.
//
// The access time is represented as four digits using a 24-hour time format, for example, "0800" or "2250".
//
// An employee is said to be high-access if he has accessed the system three or more times within a one-hour period.
//
// Times with exactly one hour of difference are not considered part of the same one-hour period.
// For example, "0815" and "0915" are not part of the same one-hour period.
//
// Access times at the start and end of the day are not counted within the same one-hour period.
// For example, "0005" and "2350" are not part of the same one-hour period.
//
// Return a list that contains the names of high-access employees with any order you want.
//
// Example 1:
//
// Input: access_times = [["a","0549"],["b","0457"],["a","0532"],["a","0621"],["b","0540"]]
// Output: ["a"]
// Explanation: "a" has three access times in the one-hour period of [05:32, 06:31] which are 05:32, 05:49, and 06:21.
// But "b" does not have more than two access times at all.
// So the answer is ["a"].
//
// Example 2:
//
// Input: access_times = [["d","0002"],["c","0808"],["c","0829"],["e","0215"],["d","1508"],["d","1444"],["d","1410"],["c","0809"]]
// Output: ["c","d"]
// Explanation: "c" has three access times in the one-hour period of [08:08, 09:07] which are 08:08, 08:09, and 08:29.
// "d" has also three access times in the one-hour period of [14:10, 15:09] which are 14:10, 14:44, and 15:08.
// However, "e" has just one access time, so it can not be in the answer and the final answer is ["c","d"].
//
// Example 3:
//
// Input: access_times = [["cd","1025"],["ab","1025"],["cd","1046"],["cd","1055"],["ab","1124"],["ab","1120"]]
// Output: ["ab","cd"]
// Explanation: "ab" has three access times in the one-hour period of [10:25, 11:24] which are 10:25, 11:20, and 11:24.
// "cd" has also three access times in the one-hour period of [10:25, 11:24] which are 10:25, 10:46, and 10:55.
// So the answer is ["ab","cd"].
//
// Constraints:
//
//     1 <= access_times.length <= 100
//     access_times[i].length == 2
//     1 <= access_times[i][0].length <= 10
//     access_times[i][0] consists only of English small letters.
//     access_times[i][1].length == 4
//     access_times[i][1] is in 24-hour time format.
//     access_times[i][1] consists only of '0' to '9'.
//

struct Solution;

impl Solution {
    pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
        let mut h = std::collections::HashMap::new();
        let mut r = Vec::new();

        for v in &access_times {
            let c = h.entry(v[0].clone()).or_insert(Vec::new());

            let ht = v[1][0..2].parse::<i32>().unwrap();
            let mt = v[1][2..4].parse::<i32>().unwrap();
            let t = 60 * ht + mt;

            c.push(t);
        }

        for (k, mut v) in h {
            v.sort();

            let mut i = 0;
            let mut j = 0;

            while j < v.len() {
                while i < j && v[j] >= v[i] + 60 {
                    i += 1;
                }

                j += 1;

                if j - i >= 3 {
                    r.push(k);
                    break;
                }
            }
        }

        r
    }
}

#[test]
fn test() {
    let access_times = vec![
        vec!["a".to_string(), "0549".to_string()],
        vec!["b".to_string(), "0457".to_string()],
        vec!["a".to_string(), "0532".to_string()],
        vec!["a".to_string(), "0621".to_string()],
        vec!["b".to_string(), "0540".to_string()],
    ];
    let output = vec!["a".to_string()];
    assert_eq!(Solution::find_high_access_employees(access_times), output);

    let access_times = vec![
        vec!["d".to_string(), "0002".to_string()],
        vec!["c".to_string(), "0808".to_string()],
        vec!["c".to_string(), "0829".to_string()],
        vec!["e".to_string(), "0215".to_string()],
        vec!["d".to_string(), "1508".to_string()],
        vec!["d".to_string(), "1444".to_string()],
        vec!["d".to_string(), "1410".to_string()],
        vec!["c".to_string(), "0809".to_string()],
    ];
    let mut r = Solution::find_high_access_employees(access_times);
    r.sort_unstable();
    let output = vec!["c".to_string(), "d".to_string()];
    assert_eq!(r, output);

    let access_times = vec![
        vec!["cd".to_string(), "1025".to_string()],
        vec!["ab".to_string(), "1025".to_string()],
        vec!["cd".to_string(), "1046".to_string()],
        vec!["cd".to_string(), "1055".to_string()],
        vec!["ab".to_string(), "1124".to_string()],
        vec!["ab".to_string(), "1120".to_string()],
    ];
    let mut r = Solution::find_high_access_employees(access_times);
    r.sort_unstable();
    let output = vec!["ab".to_string(), "cd".to_string()];
    assert_eq!(r, output);
}
