#![allow(dead_code)]

/*

// 2069. Walking Robot Simulation II
// https://leetcode.com/problems/walking-robot-simulation-ii/
// https://leetcode.cn/problems/walking-robot-simulation-ii/
//
// Medium
//
// A width x height grid is on an XY-plane with the bottom-left cell at (0, 0) and the top-right cell at (width - 1, height - 1). The grid is aligned with the four cardinal directions ("North", "East", "South", and "West"). A robot is initially at cell (0, 0) facing direction "East".

The robot can be instructed to move for a specific number of steps. For each step, it does the following.

    Attempts to move forward one cell in the direction it is facing.
    If the cell the robot is moving to is out of bounds, the robot instead turns 90 degrees counterclockwise and retries the step.

After the robot finishes moving the number of steps required, it stops and awaits the next instruction.

Implement the Robot class:

    Robot(int width, int height) Initializes the width x height grid with the robot at (0, 0) facing "East".
    void step(int num) Instructs the robot to move forward num steps.
    int[] getPos() Returns the current cell the robot is at, as an array of length 2, [x, y].
    String getDir() Returns the current direction of the robot, "North", "East", "South", or "West".

Example 1:
example-1

Input
["Robot", "step", "step", "getPos", "getDir", "step", "step", "step", "getPos", "getDir"]
[[6, 3], [2], [2], [], [], [2], [1], [4], [], []]
Output
[null, null, null, [4, 0], "East", null, null, null, [1, 2], "West"]

Explanation
Robot robot = new Robot(6, 3); // Initialize the grid and the robot at (0, 0) facing East.
robot.step(2);  // It moves two steps East to (2, 0), and faces East.
robot.step(2);  // It moves two steps East to (4, 0), and faces East.
robot.getPos(); // return [4, 0]
robot.getDir(); // return "East"
robot.step(2);  // It moves one step East to (5, 0), and faces East.
                // Moving the next step East would be out of bounds, so it turns and faces North.
                // Then, it moves one step North to (5, 1), and faces North.
robot.step(1);  // It moves one step North to (5, 2), and faces North (not West).
robot.step(4);  // Moving the next step North would be out of bounds, so it turns and faces West.
                // Then, it moves four steps West to (1, 2), and faces West.
robot.getPos(); // return [1, 2]
robot.getDir(); // return "West"

Constraints:

    2 <= width, height <= 100
    1 <= num <= 10^5
    At most 104 calls in total will be made to step, getPos, and getDir.
*/

struct Robot {
    dir: Vec<Vec<i32>>,
    dir_str: Vec<String>,
    pos: Vec<i32>,
    w: i32,
    h: i32,
    idx: usize,
    count: i32,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Robot {
            dir: vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]],
            dir_str: vec![
                "East".to_string(),
                "North".to_string(),
                "West".to_string(),
                "South".to_string(),
            ],
            pos: vec![0, 0],
            w: width,
            h: height,
            idx: 0,
            count: 0,
        }
    }

    fn step(&mut self, num: i32) {
        self.count = (self.count + num) % (2 * (self.w + self.h - 2));
        if self.count >= 0 && self.count < self.w {
            self.pos = vec![self.count, 0];
            self.idx = if self.count == 0 { 3 } else { 0 };
        } else if self.count >= self.w && self.count < self.w + self.h - 1 {
            self.pos = vec![self.w - 1, self.count - self.w + 1];
            self.idx = 1;
        } else if self.count >= self.w + self.h - 1 && self.count < 2 * self.w + self.h - 2 {
            self.pos = vec![2 * self.w + self.h - self.count - 3, self.h - 1];
            self.idx = 2;
        } else {
            self.pos = vec![0, 2 * self.h + 2 * self.w - self.count - 4];
            self.idx = 3;
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        self.pos.clone()
    }

    fn get_dir(&self) -> String {
        self.dir_str[self.idx].clone()
    }
}

#[test]
fn test() {
    let mut robot = Robot::new(6, 3);
    robot.step(2);
    robot.step(2);
    assert_eq!(robot.get_pos(), vec![4, 0]);
    assert_eq!(robot.get_dir(), "East");
    robot.step(2);
    robot.step(1);
    robot.step(4);
    assert_eq!(robot.get_pos(), vec![1, 2]);
    assert_eq!(robot.get_dir(), "West");
}
