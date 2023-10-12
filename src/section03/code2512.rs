#![allow(dead_code)]

// 2512. Reward Top K Students
// https://leetcode.com/problems/reward-top-k-students/
// https://leetcode.cn/problems/reward-top-k-students/
//
// You are given two string arrays positive_feedback and negative_feedback, containing the words denoting positive and negative feedback, respectively. Note that no word is both positive and negative.
//
// Initially every student has 0 points. Each positive word in a feedback report increases the points of a student by 3, whereas each negative word decreases the points by 1.
//
// You are given n feedback reports, represented by a 0-indexed string array report and a 0-indexed integer array student_id, where student_id[i] represents the ID of the student who has received the feedback report report[i]. The ID of each student is unique.
//
// Given an integer k, return the top k students after ranking them in non-increasing order by their points. In case more than one student has the same points, the one with the lower ID ranks higher.
//
// Example 1:
//
// Input: positive_feedback = ["smart","brilliant","studious"], negative_feedback = ["not"], report = ["this student is studious","the student is smart"], student_id = [1,2], k = 2
// Output: [1,2]
// Explanation:
// Both the students have 1 positive feedback and 3 points but since student 1 has a lower ID he ranks higher.
//
// Example 2:
//
// Input: positive_feedback = ["smart","brilliant","studious"], negative_feedback = ["not"], report = ["this student is not studious","the student is smart"], student_id = [1,2], k = 2
// Output: [2,1]
// Explanation:
// - The student with ID 1 has 1 positive feedback and 1 negative feedback, so he has 3-1=2 points.
// - The student with ID 2 has 1 positive feedback, so he has 3 points.
// Since student 2 has more points, [2,1] is returned.
//
// Constraints:
//
// - 1 <= positive_feedback.length, negative_feedback.length <= 10^4
// - 1 <= positive_feedback[i].length, negative_feedback[j].length <= 100
// - Both positive_feedback[i] and negative_feedback[j] consists of lowercase English letters.
// - No word is present in both positive_feedback and negative_feedback.
// - n == report.length == student_id.length
// - 1 <= n <= 10^4
// - report[i] consists of lowercase English letters and spaces ' '.
// - There is a single space between consecutive words of report[i].
// - 1 <= report[i].length <= 100
// - 1 <= student_id[i] <= 10^9
// - All the values of student_id[i] are unique.
// - 1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        use std::collections::HashSet;
        let positive_feedback = positive_feedback.into_iter().collect::<HashSet<_>>();
        let negative_feedback = negative_feedback.into_iter().collect::<HashSet<_>>();
        let mut mylist = vec![];
        for (i, r) in report.iter().enumerate() {
            let mut score = 0;
            let s = r.split(' ');
            for st in s {
                if positive_feedback.contains(st) {
                    score += 3;
                } else if negative_feedback.contains(st) {
                    score -= 1;
                }
            }
            mylist.push((score, student_id[i]));
        }
        mylist.sort_by_key(|x| (x.0, -x.1));
        mylist.reverse();
        mylist.into_iter().take(k as usize).map(|x| x.1).collect::<Vec<_>>()
    }
}

#[test]
fn test() {
    let positive_feedback = vec!["smart".to_string(), "brilliant".to_string(), "studious".to_string()];
    let negative_feedback = vec!["not".to_string()];
    let report = vec!["this student is studious".to_string(), "the student is smart".to_string()];
    let student_id = vec![1, 2];
    let k = 2;
    let ret = Solution::top_students(positive_feedback, negative_feedback, report, student_id, k);
    assert_eq!(ret, vec![1, 2]);

    let positive_feedback = vec!["smart".to_string(), "brilliant".to_string(), "studious".to_string()];
    let negative_feedback = vec!["not".to_string()];
    let report = vec!["this student is not studious".to_string(), "the student is smart".to_string()];
    let student_id = vec![1, 2];
    let k = 2;
    let ret = Solution::top_students(positive_feedback, negative_feedback, report, student_id, k);
    assert_eq!(ret, vec![2, 1]);

    let positive_feedback = ["fkeofjpc", "qq", "iio"];
    let positive_feedback = positive_feedback.iter().map(|s| s.to_string()).collect();
    let negative_feedback = ["jdh", "khj", "eget", "rjstbhe", "yzyoatfyx", "wlinrrgcm"];
    let negative_feedback = negative_feedback.iter().map(|s| s.to_string()).collect();
    let report = [
        "rjstbhe eget kctxcoub urrmkhlmi yniqafy fkeofjpc iio yzyoatfyx khj iio",
        "gpnhgabl qq qq fkeofjpc dflidshdb qq iio khj qq yzyoatfyx",
        "tizpzhlbyb eget z rjstbhe iio jdh jdh iptxh qq rjstbhe",
        "jtlghe wlinrrgcm jnkdbd k iio et rjstbhe iio qq jdh",
        "yp fkeofjpc lkhypcebox rjstbhe ewwykishv egzhne jdh y qq qq",
        "fu ql iio fkeofjpc jdh luspuy yzyoatfyx li qq v",
        "wlinrrgcm iio qq omnc sgkt tzgev iio iio qq qq",
        "d vhg qlj khj wlinrrgcm qq f jp zsmhkjokmb rjstbhe",
    ];
    let report = report.iter().map(|s| s.to_string()).collect();
    let student_id = vec![96537918, 589204657, 765963609, 613766496, 43871615, 189209587, 239084671, 908938263];
    let k = 3;
    let ret = Solution::top_students(positive_feedback, negative_feedback, report, student_id, k);
    assert_eq!(ret, vec![239084671, 589204657, 43871615]);
}
