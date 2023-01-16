#![allow(dead_code)]

// 855. Exam Room
// https://leetcode.com/problems/exam-room/
// https://leetcode.cn/problems/exam-room/
//
// There is an exam room with n seats in a single row labeled from 0 to n - 1.
//
// When a student enters the room, they must sit in the seat that maximizes the distance to the closest person.
// If there are multiple such seats, they sit in the seat with the lowest number.
// If no one is in the room, then the student sits at seat number 0.
//
// Design a class that simulates the mentioned exam room.
//
// Implement the ExamRoom class:
//
// ExamRoom(int n) Initializes the object of the exam room with the number of the seats n.
// int seat() Returns the label of the seat at which the next student will set.
// void leave(int p) Indicates that the student sitting at seat p will leave the room.
//       It is guaranteed that there will be a student sitting at seat p.
//
// Example 1:
//
// Input
// ["ExamRoom", "seat", "seat", "seat", "seat", "leave", "seat"]
// [[10], [], [], [], [], [4], []]
// Output
// [null, 0, 9, 4, 2, null, 5]
//
// Explanation
// ExamRoom examRoom = new ExamRoom(10);
// examRoom.seat(); // return 0, no one is in the room, then the student sits at seat number 0.
// examRoom.seat(); // return 9, the student sits at the last seat number 9.
// examRoom.seat(); // return 4, the student sits at the last seat number 4.
// examRoom.seat(); // return 2, the student sits at the last seat number 2.
// examRoom.leave(4);
// examRoom.seat(); // return 5, the student sits at the last seat number 5.
//
// Constraints:
//
// - 1 <= n <= 10^9
// - It is guaranteed that there is a student sitting at seat p.
// - At most 104 calls will be made to seat and leave.
//

use std::collections::BTreeSet;

struct ExamRoom {
    n: i32,
    seats: BTreeSet<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            n,
            seats: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        let mut max_dist = 0;
        let mut seat = 0;
        let mut prev = -1;
        for &s in &self.seats {
            let dist = if prev == -1 { s } else { (s - prev) / 2 };
            if dist > max_dist {
                max_dist = dist;
                seat = if prev == -1 { 0 } else { prev + dist };
            }
            prev = s;
        }
        if prev != -1 && self.n - 1 - prev > max_dist {
            seat = self.n - 1;
        }
        self.seats.insert(seat);
        seat
    }

    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}

#[test]
fn test() {
    let mut exam_room = ExamRoom::new(10);
    assert_eq!(exam_room.seat(), 0);
    assert_eq!(exam_room.seat(), 9);
    assert_eq!(exam_room.seat(), 4);
    assert_eq!(exam_room.seat(), 2);
    exam_room.leave(4);
    assert_eq!(exam_room.seat(), 5);
}
