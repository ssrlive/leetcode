#![allow(dead_code)]

/*

// 1622. Fancy Sequence
// https://leetcode.com/problems/fancy-sequence/
// https://leetcode.cn/problems/fancy-sequence/
//
// Hard
//
// Write an API that generates fancy sequences using the append, addAll, and multAll operations.

Implement the Fancy class:

    Fancy() Initializes the object with an empty sequence.
    void append(val) Appends an integer val to the end of the sequence.
    void addAll(inc) Increments all existing values in the sequence by an integer inc.
    void multAll(m) Multiplies all existing values in the sequence by an integer m.
    int getIndex(idx) Gets the current value at index idx (0-indexed) of the sequence modulo 109 + 7. If the index is greater or equal than the length of the sequence, return -1.

Example 1:

Input
["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll", "getIndex", "getIndex", "getIndex"]
[[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]]
Output
[null, null, null, null, null, 10, null, null, null, 26, 34, 20]

Explanation
Fancy fancy = new Fancy();
fancy.append(2);   // fancy sequence: [2]
fancy.addAll(3);   // fancy sequence: [2+3] -> [5]
fancy.append(7);   // fancy sequence: [5, 7]
fancy.multAll(2);  // fancy sequence: [5*2, 7*2] -> [10, 14]
fancy.getIndex(0); // return 10
fancy.addAll(3);   // fancy sequence: [10+3, 14+3] -> [13, 17]
fancy.append(10);  // fancy sequence: [13, 17, 10]
fancy.multAll(2);  // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
fancy.getIndex(0); // return 26
fancy.getIndex(1); // return 34
fancy.getIndex(2); // return 20

Constraints:

    1 <= val, inc, m <= 100
    0 <= idx <= 10^5
    At most 105 calls total will be made to append, addAll, multAll, and getIndex.
*/

/*
class Fancy {
public:
    unsigned long seq[100001];
    unsigned int length = 0;
    unsigned long increment = 0;
    unsigned long mult = 1;

    const int mod97 = 1000000007;

    Fancy() {
        ios_base::sync_with_stdio(false);
        cin.tie(NULL);
    }

    void append(int val) {
        seq[length++] = (((mod97 + val - increment)%mod97) * modPow(mult, mod97-2))%mod97;
    }
    void addAll(int inc) {
        increment = (increment+ inc%mod97)%mod97;
    }

    void multAll(int m) {
        mult = (mult* m%mod97)%mod97;
        increment = (increment* m%mod97)%mod97;
    }

    int getIndex(int idx) {

        if (idx >= length){
            return -1;
        }else{
            return ((seq[idx] * mult)%mod97+increment)%mod97;
        }
    }

    unsigned long modPow(unsigned long x, int y) {
        unsigned long tot = 1, p = x;
        for (; y; y >>= 1) {
            if (y & 1)
                tot = (tot * p) % mod97;
            p = (p * p) % mod97;
        }
        return tot;
    }
};
 */

const MOD: i64 = 1_000_000_007;

struct Fancy {
    seq: Vec<i64>,
    increment: i64,
    mult: i64,
}

impl Fancy {
    fn new() -> Self {
        Fancy {
            seq: Vec::new(),
            increment: 0,
            mult: 1,
        }
    }

    fn append(&mut self, val: i32) {
        let v = (((MOD + val as i64 - self.increment) % MOD) * self.mod_pow(self.mult, MOD - 2)) % MOD;
        self.seq.push(v);
    }

    fn add_all(&mut self, inc: i32) {
        self.increment = (self.increment + inc as i64) % MOD;
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mult = (self.mult * m % MOD) % MOD;
        self.increment = (self.increment * m % MOD) % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.seq.len() {
            -1
        } else {
            let v = ((self.seq[idx] * self.mult) % MOD + self.increment) % MOD;
            v as _
        }
    }

    fn mod_pow(&self, x: i64, y: i64) -> i64 {
        let mut tot = 1;
        let mut p = x;
        let mut y = y;
        while y > 0 {
            if y & 1 == 1 {
                tot = (tot * p) % MOD;
            }
            p = (p * p) % MOD;
            y >>= 1;
        }
        tot
    }
}

#[test]
fn test() {
    let mut fancy = Fancy::new();
    fancy.append(2);
    fancy.add_all(3);
    fancy.append(7);
    fancy.mult_all(2);
    assert_eq!(fancy.get_index(0), 10);
    fancy.add_all(3);
    fancy.append(10);
    fancy.mult_all(2);
    assert_eq!(fancy.get_index(0), 26);
    assert_eq!(fancy.get_index(1), 34);
    assert_eq!(fancy.get_index(2), 20);
}
