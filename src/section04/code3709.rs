#![allow(dead_code)]

// 3709. Design Exam Scores Tracker
// https://leetcode.com/problems/design-exam-scores-tracker/
// https://leetcode.cn/problems/design-exam-scores-tracker/
//
// Medium
//
// Alice frequently takes exams and wants to track her scores and calculate the total scores over specific time periods.
//
// Implement the ExamTracker class:
//
// - ExamTracker(): Initializes the ExamTracker object.
// - void record(int time, int score): Alice takes a new exam at time time and achieves the score score.
// - long long totalScore(int startTime, int endTime): Returns an integer that represents the total score
//   of all exams taken by Alice between startTime and endTime (inclusive).
//   If there are no recorded exams taken by Alice within the specified time interval, return 0.
// - It is guaranteed that the function calls are made in chronological order. That is,
//
// Calls to record() will be made with strictly increasing time.
// Alice will never ask for total scores that require information from the future.
// That is, if the latest record() is called with time = t, then totalScore() will always be called with startTime <= endTime <= t.
//
// Example 1:
//
// Input:
// ["ExamTracker", "record", "totalScore", "record", "totalScore", "totalScore", "totalScore", "totalScore"]
// [[], [1, 98], [1, 1], [5, 99], [1, 3], [1, 5], [3, 4], [2, 5]]
//
// Output:
// [null, null, 98, null, 98, 197, 0, 99]
//
// Explanation
//
// ExamTracker examTracker = new ExamTracker();
// examTracker.record(1, 98); // Alice takes a new exam at time 1, scoring 98.
// examTracker.totalScore(1, 1); // Between time 1 and time 1, Alice took 1 exam at time 1, scoring 98. The total score is 98.
// examTracker.record(5, 99); // Alice takes a new exam at time 5, scoring 99.
// examTracker.totalScore(1, 3); // Between time 1 and time 3, Alice took 1 exam at time 1, scoring 98. The total score is 98.
// examTracker.totalScore(1, 5); // Between time 1 and time 5, Alice took 2 exams at time 1 and 5, scoring 98 and 99. The total score is 98 + 99 = 197.
// examTracker.totalScore(3, 4); // Alice did not take any exam between time 3 and time 4. Therefore, the answer is 0.
// examTracker.totalScore(2, 5); // Between time 2 and time 5, Alice took 1 exam at time 5, scoring 99. The total score is 99.
//
// Constraints:
//
// 1 <= time <= 10^9
// 1 <= score <= 10^9
// 1 <= startTime <= endTime <= t, where t is the value of time from the most recent call of record().
// Calls of record() will be made with strictly increasing time.
// After ExamTracker(), the first function call will always be record().
// At most 10^5 calls will be made in total to record() and totalScore().
//

struct ExamTracker {
    time_score: Vec<(i32, i64)>,
}

impl ExamTracker {
    fn new() -> Self {
        ExamTracker { time_score: vec![(0, 0)] }
    }

    fn record(&mut self, time: i32, score: i32) {
        let sum = score as i64 + self.time_score.last().unwrap().1;
        self.time_score.push((time, sum));
    }

    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let left = self.time_score.binary_search_by(|t| t.0.cmp(&start_time)).unwrap_or_else(|x| x);

        let right = self
            .time_score
            .binary_search_by(|t| match t.0.cmp(&end_time) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            })
            .unwrap_or_else(|x| x);

        self.time_score[right - 1].1 - self.time_score[left - 1].1
    }
}

#[test]
fn test() {
    let mut exam_tracker = ExamTracker::new();
    exam_tracker.record(1, 98);
    assert_eq!(exam_tracker.total_score(1, 1), 98);
    exam_tracker.record(5, 99);
    assert_eq!(exam_tracker.total_score(1, 3), 98);
    assert_eq!(exam_tracker.total_score(1, 5), 197);
    assert_eq!(exam_tracker.total_score(3, 4), 0);
    assert_eq!(exam_tracker.total_score(2, 5), 99);
}
