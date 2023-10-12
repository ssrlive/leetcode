#![allow(dead_code)]

/*

// 1792. Maximum Average Pass Ratio
Medium
627
68
Companies

There is a school that has classes of students and each class will be having a final exam. You are given a 2D integer array classes, where classes[i] = [passi, totali]. You know beforehand that in the ith class, there are totali total students, but only passi number of students will pass the exam.

You are also given an integer extraStudents. There are another extraStudents brilliant students that are guaranteed to pass the exam of any class they are assigned to. You want to assign each of the extraStudents students to a class in a way that maximizes the average pass ratio across all the classes.

The pass ratio of a class is equal to the number of students of the class that will pass the exam divided by the total number of students of the class. The average pass ratio is the sum of pass ratios of all the classes divided by the number of the classes.

Return the maximum possible average pass ratio after assigning the extraStudents students. Answers within 10-5 of the actual answer will be accepted.

Example 1:

Input: classes = [[1,2],[3,5],[2,2]], extraStudents = 2
Output: 0.78333
Explanation: You can assign the two extra students to the first class. The average pass ratio will be equal to (3/4 + 3/5 + 2/2) / 3 = 0.78333.

Example 2:

Input: classes = [[2,4],[3,9],[4,5],[2,10]], extraStudents = 4
Output: 0.53485

Constraints:

    1 <= classes.length <= 10^5
    classes[i].length == 2
    1 <= passi <= totali <= 10^5
    1 <= extraStudents <= 10^5
*/

struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        fn profit(pass: i32, total: i32) -> f64 {
            (pass + 1) as f64 / (total + 1) as f64 - pass as f64 / total as f64
        }

        let mut total = 0.0;
        let mut pq = std::collections::BinaryHeap::new();
        for c in classes.iter() {
            total += c[0] as f64 / c[1] as f64;
            let v = profit(c[0], c[1]);
            pq.push((OrderedF64(v), c[0], c[1]));
        }
        for _ in 0..extra_students {
            let (added_profit, c0, c1) = pq.pop().unwrap();
            total += added_profit.0;
            let v = profit(c0 + 1, c1 + 1);
            pq.push((OrderedF64(v), c0 + 1, c1 + 1));
        }
        total / classes.len() as f64
    }

    pub fn max_average_ratio2(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let classes = classes.into_iter().map(|c| (c[0], c[1])).collect::<Vec<(i32, i32)>>();
        let mut pq = std::collections::BinaryHeap::new();
        for (pass, total) in classes.iter() {
            let ratio = (pass + 1) as f64 / (total + 1) as f64 - *pass as f64 / *total as f64;
            pq.push((OrderedF64(ratio), *pass, *total));
        }
        for _ in 0..extra_students {
            let (_, pass, total) = pq.pop().unwrap();
            let ratio = (pass + 2) as f64 / (total + 2) as f64 - (pass + 1) as f64 / (total + 1) as f64;
            pq.push((OrderedF64(ratio), pass + 1, total + 1));
        }
        let mut res = 0.0;
        for (_, pass, total) in pq.into_iter() {
            res += pass as f64 / total as f64;
        }
        res / classes.len() as f64
    }
}

#[derive(Debug, Clone)]
struct OrderedF64(f64);

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
impl PartialOrd for OrderedF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for OrderedF64 {}

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[test]
fn test() {
    let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
    let extra_students = 2;
    let res = 0.7833333333333333;
    assert_eq!(Solution::max_average_ratio2(classes, extra_students), res);
    let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
    let extra_students = 4;
    let res = 0.5348484848484848;
    assert_eq!(Solution::max_average_ratio2(classes, extra_students), res);
}
