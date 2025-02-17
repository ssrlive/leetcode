#![allow(dead_code)]

// 3454. Separate Squares II
// https://leetcode.com/problems/separate-squares-ii/
// https://leetcode.cn/problems/separate-squares-ii/
//
// Hard
//
// You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left
// point and the side length of a square parallel to the x-axis.
//
// Find the minimum y-coordinate value of a horizontal line such that the total area covered by squares above the line
// equals the total area covered by squares below the line.
//
// Answers within 10-5 of the actual answer will be accepted.
//
// Note: Squares may overlap. Overlapping areas should be counted only once in this version.
//
// Example 1:
//
// Input: squares = [[0,0,1],[2,2,1]]
//
// Output: 1.00000
//
// Explanation:
//
// Any horizontal line between y = 1 and y = 2 results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
//
// Example 2:
//
// Input: squares = [[0,0,2],[1,1,1]]
//
// Output: 1.00000
//
// Explanation:
//
// Since the blue square overlaps with the red square, it will not be counted again. Thus, the line y = 1 splits the squares into two equal parts.
//
// Constraints:
//
//     1 <= squares.length <= 5 * 10^4
//     squares[i] = [xi, yi, li]
//     squares[i].length == 3
//     0 <= xi, yi <= 10^9
//     1 <= li <= 10^9
//

struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        const START: i64 = 1;
        const END: i64 = -1;

        let squares: Vec<Vec<i64>> = squares.iter().map(|sq| sq.iter().map(|&x| x as i64).collect()).collect();

        let mut mapping = std::collections::BTreeMap::new();
        let mut vals = std::collections::BTreeSet::new();
        for sq in squares.iter() {
            vals.insert(sq[0] - 1);
            vals.insert(sq[0]);
            vals.insert(sq[0] + sq[2] - 1);
            vals.insert(sq[0] + sq[2]);
        }
        for (i, val) in vals.iter().enumerate() {
            mapping.insert(*val, i as i64);
        }

        let mut events = Vec::new();
        for sq in squares.iter() {
            let mapping_x1 = sq[0];
            let mapping_x2 = sq[0] + sq[2] - 1;

            let x1 = *mapping.get(&mapping_x1).unwrap();
            let x2 = *mapping.get(&mapping_x2).unwrap();

            events.push(Event::new(x1, x2, sq[1], START));
            events.push(Event::new(x1, x2, sq[1] + sq[2], END));
        }

        events.sort_by(|a, b| a.y.cmp(&b.y));

        let mut segtree = SegmentTree::new(&vals);

        let mut prev_y = 0;
        let mut total_area = 0;
        for event in events.iter() {
            total_area += (event.y - prev_y) * segtree.covered();
            prev_y = event.y;
            segtree.update_range(event.x1, event.x2, event.event_type);
        }

        let mut current_area = 0;
        prev_y = 0;
        for event in events.iter() {
            let next_area = current_area + (event.y - prev_y) * segtree.covered();
            if next_area >= (total_area + 1) / 2 {
                return prev_y as f64 + (total_area as f64 / 2.0 - current_area as f64) / segtree.covered() as f64;
            } else {
                current_area = next_area;
            }
            prev_y = event.y;
            segtree.update_range(event.x1, event.x2, event.event_type);
        }
        panic!("impossible");
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Event {
    x1: i64,
    x2: i64,
    y: i64,
    event_type: i64,
}
impl Event {
    fn new(x1: i64, x2: i64, y: i64, event_type: i64) -> Self {
        Self { x1, x2, y, event_type }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Range {
    x1: i64,
    x2: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct Node {
    covered_cnt: i64,
    covered_len: i64,
    left_covered: bool,
    right_covered: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SegmentTree {
    vals: Vec<i64>,
    tree: Vec<Node>,
    sz: i64,
}

impl SegmentTree {
    pub fn new(st: &std::collections::BTreeSet<i64>) -> Self {
        let sz = st.len() as i64;
        let mut vals = Vec::new();
        vals.extend(st.iter().copied());
        let tree = vec![Node::default(); (4 * sz) as usize];
        Self { vals, tree, sz }
    }

    pub fn update_range(&mut self, l: i64, r: i64, delta: i64) {
        self.update_range_inner(l as usize, r as usize, delta, 1, 0, (self.sz - 1) as usize);
    }

    fn update_range_inner(&mut self, l: usize, r: usize, delta: i64, v: usize, vl: usize, vr: usize) {
        if vl > r || vr < l {
            return;
        }

        let mid = (vl + vr) / 2;
        if l <= vl && vr <= r {
            self.tree[v].covered_cnt += delta;
        } else {
            self.update_range_inner(l, r, delta, 2 * v, vl, mid);
            self.update_range_inner(l, r, delta, 2 * v + 1, mid + 1, vr);
        }
        if self.tree[v].covered_cnt != 0 {
            self.tree[v].covered_len = self.vals[vr] - self.vals[vl] + 1;
            self.tree[v].left_covered = true;
            self.tree[v].right_covered = true;
        } else if vl == vr {
            self.tree[v].covered_len = 0;
            self.tree[v].left_covered = false;
            self.tree[v].right_covered = false;
        } else {
            self.tree[v].covered_len = self.tree[2 * v].covered_len + self.tree[2 * v + 1].covered_len;
            if self.tree[2 * v].right_covered && self.tree[2 * v + 1].left_covered {
                self.tree[v].covered_len += self.vals[mid + 1] - self.vals[mid] - 1;
            }
            self.tree[v].left_covered = self.tree[2 * v].left_covered;
            self.tree[v].right_covered = self.tree[2 * v + 1].right_covered;
        }
    }

    pub fn covered(&self) -> i64 {
        self.tree[1].covered_len
    }
}

#[test]
fn test() {
    let squares = vec![vec![0, 0, 1], vec![2, 2, 1]];
    let res = 1.0;
    assert_eq!(Solution::separate_squares(squares), res);

    let squares = vec![vec![0, 0, 2], vec![1, 1, 1]];
    let res = 1.0;
    assert_eq!(Solution::separate_squares(squares), res);

    let squares = vec![vec![15, 21, 2], vec![19, 21, 3]];
    let res = 22.3;
    assert_eq!(Solution::separate_squares(squares), res);

    let squares = vec![
        vec![999892931, 999974790, 434471746],
        vec![319710671, 963660807, 875442433],
        vec![353202089, 976938743, 622045959],
        vec![765760000, 939956921, 271907109],
        vec![234214719, 848813522, 26688635],
        vec![154771654, 645515409, 804966565],
        vec![599682863, 948151649, 645372386],
        vec![534582408, 880488308, 338822523],
        vec![211765761, 379959124, 514093466],
        vec![331072193, 790928668, 912992282],
    ];
    let res = 1173798088.1406226;
    assert_eq!(Solution::separate_squares(squares), res);
}
