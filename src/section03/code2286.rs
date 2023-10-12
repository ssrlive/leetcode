#![allow(dead_code)]

/*

// 2286. Booking Concert Tickets in Groups
// https://leetcode.com/problems/booking-concert-tickets-in-groups/
// https://leetcode.cn/problems/booking-concert-tickets-in-groups/
//
// Hard
//
// A concert hall has n rows numbered from 0 to n - 1, each with m seats, numbered from 0 to m - 1.
// You need to design a ticketing system that can allocate seats in the following cases:

    If a group of k spectators can sit together in a row.
    If every member of a group of k spectators can get a seat. They may or may not sit together.

Note that the spectators are very picky. Hence:

    They will book seats only if each member of their group can get a seat with row number less than or equal to maxRow. maxRow can vary from group to group.
    In case there are multiple rows to choose from, the row with the smallest number is chosen. If there are multiple seats to choose in the same row, the seat with the smallest number is chosen.

Implement the BookMyShow class:

    BookMyShow(int n, int m) Initializes the object with n as number of rows and m as number of seats per row.
    int[] gather(int k, int maxRow) Returns an array of length 2 denoting the row and seat number (respectively) of the first seat being allocated to the k members of the group, who must sit together. In other words, it returns the smallest possible r and c such that all [c, c + k - 1] seats are valid and empty in row r, and r <= maxRow. Returns [] in case it is not possible to allocate seats to the group.
    boolean scatter(int k, int maxRow) Returns true if all k members of the group can be allocated seats in rows 0 to maxRow, who may or may not sit together. If the seats can be allocated, it allocates k seats to the group with the smallest row numbers, and the smallest possible seat numbers in each row. Otherwise, returns false.

Example 1:

Input
["BookMyShow", "gather", "gather", "scatter", "scatter"]
[[2, 5], [4, 0], [2, 0], [5, 1], [5, 1]]
Output
[null, [0, 0], [], true, false]

Explanation
BookMyShow bms = new BookMyShow(2, 5); // There are 2 rows with 5 seats each
bms.gather(4, 0); // return [0, 0]
                  // The group books seats [0, 3] of row 0.
bms.gather(2, 0); // return []
                  // There is only 1 seat left in row 0,
                  // so it is not possible to book 2 consecutive seats.
bms.scatter(5, 1); // return True
                   // The group books seat 4 of row 0 and seats [0, 3] of row 1.
bms.scatter(5, 1); // return False
                   // There is only one seat left in the hall.

Constraints:

    1 <= n <= 5 * 10^4
    1 <= m, k <= 10^9
    0 <= maxRow <= n - 1
    At most 5 * 10^4 calls in total will be made to gather and scatter.
*/

struct BookMyShow {
    tree: Vec<(i32, i64)>,
    m: usize,
    n: usize,
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let mut temp: Vec<(i32, i64)> = vec![(0, 0); 4 * n as usize];
        Self::build(1, 0, n as usize - 1, &mut temp, m as usize);
        BookMyShow {
            tree: temp,
            m: m as usize,
            n: n as usize,
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        self.gather_detail(1, 0, self.n - 1, k, max_row as usize)
    }

    fn gather_detail(&mut self, u: usize, left: usize, right: usize, k: i32, max_row: usize) -> Vec<i32> {
        if left > max_row || self.tree[u].0 < k {
            return vec![];
        }

        if left == right {
            self.tree[u].0 -= k;
            self.tree[u].1 -= k as i64;
            return vec![left as i32, self.m as i32 - self.tree[u].0 - k];
        }

        let mid = left + (right - left) / 2;
        let ret: Vec<i32> = if self.tree[2 * u].0 >= k {
            self.gather_detail(2 * u, left, mid, k, max_row)
        } else {
            self.gather_detail(2 * u + 1, mid + 1, right, k, max_row)
        };

        self.tree[u].0 = self.tree[2 * u].0.max(self.tree[2 * u + 1].0);
        self.tree[u].1 = self.tree[2 * u].1 + self.tree[2 * u + 1].1;
        ret
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        if !self.check(1, 0, self.n - 1, k, max_row as usize) {
            return false;
        }
        self.scatter_detail(1, 0, self.n - 1, k);
        true
    }

    fn check(&self, u: usize, left: usize, right: usize, k: i32, max_row: usize) -> bool {
        if left > max_row || self.tree[u].1 < k as i64 {
            return false;
        }
        if left == right {
            return true;
        }

        let mid = left + (right - left) / 2;
        if mid + 1 > max_row {
            return self.check(2 * u, left, mid, k, max_row);
        }

        if k as i64 <= self.tree[2 * u].1 {
            return true;
        }
        self.check(2 * u + 1, mid + 1, right, k - self.tree[2 * u].1 as i32, max_row)
    }

    fn scatter_detail(&mut self, u: usize, left: usize, right: usize, k: i32) {
        if left == right {
            self.tree[u].0 -= k;
            self.tree[u].1 -= k as i64;
            return;
        }

        let mid = left + (right - left) / 2;
        let remaining = k as i64 - self.tree[2 * u].1;
        self.scatter_detail(2 * u, left, mid, self.tree[2 * u].1.min(k as i64) as i32);
        if remaining > 0 {
            self.scatter_detail(2 * u + 1, mid + 1, right, remaining as i32);
        }

        self.tree[u].0 = i32::max(self.tree[2 * u].0, self.tree[2 * u + 1].0);
        self.tree[u].1 = self.tree[2 * u].1 + self.tree[2 * u + 1].1;
    }

    fn build(u: usize, left: usize, right: usize, v: &mut Vec<(i32, i64)>, m: usize) {
        if left == right {
            v[u] = (m as i32, m as i64);
            return;
        }

        let mid = left + (right - left) / 2;
        Self::build(2 * u, left, mid, v, m);
        Self::build(2 * u + 1, mid + 1, right, v, m);

        let a = v[2 * u].0.max(v[2 * u + 1].0);
        let b = v[2 * u].1 + v[2 * u + 1].1;
        v[u] = (a, b);
    }
}

#[test]
fn test() {
    let mut bms = BookMyShow::new(2, 5);
    assert_eq!(bms.gather(4, 0), vec![0, 0]);
    assert_eq!(bms.gather(2, 0), vec![]);
    assert!(bms.scatter(5, 1));
    assert!(!bms.scatter(5, 1));
}
