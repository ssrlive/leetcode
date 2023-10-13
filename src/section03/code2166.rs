#![allow(dead_code)]

/*

// 2166. Design Bitset
// https://leetcode.com/problems/design-bitset/
// https://leetcode.cn/problems/design-bitset/
//
// Medium
//
// A Bitset is a data structure that compactly stores bits.

Implement the Bitset class:

    Bitset(int size) Initializes the Bitset with size bits, all of which are 0.
    void fix(int idx) Updates the value of the bit at the index idx to 1. If the value was already 1, no change occurs.
    void unfix(int idx) Updates the value of the bit at the index idx to 0. If the value was already 0, no change occurs.
    void flip() Flips the values of each bit in the Bitset. In other words, all bits with value 0 will now have value 1 and vice versa.
    boolean all() Checks if the value of each bit in the Bitset is 1. Returns true if it satisfies the condition, false otherwise.
    boolean one() Checks if there is at least one bit in the Bitset with value 1. Returns true if it satisfies the condition, false otherwise.
    int count() Returns the total number of bits in the Bitset which have value 1.
    String toString() Returns the current composition of the Bitset. Note that in the resultant string, the character at the ith index should coincide with the value at the ith bit of the Bitset.

Example 1:

Input
["Bitset", "fix", "fix", "flip", "all", "unfix", "flip", "one", "unfix", "count", "toString"]
[[5], [3], [1], [], [], [0], [], [], [0], [], []]
Output
[null, null, null, null, false, null, null, true, null, 2, "01010"]

Explanation
Bitset bs = new Bitset(5); // bitset = "00000".
bs.fix(3);     // the value at idx = 3 is updated to 1, so bitset = "00010".
bs.fix(1);     // the value at idx = 1 is updated to 1, so bitset = "01010".
bs.flip();     // the value of each bit is flipped, so bitset = "10101".
bs.all();      // return False, as not all values of the bitset are 1.
bs.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "00101".
bs.flip();     // the value of each bit is flipped, so bitset = "11010".
bs.one();      // return True, as there is at least 1 index with value 1.
bs.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "01010".
bs.count();    // return 2, as there are 2 bits with value 1.
bs.toString(); // return "01010", which is the composition of bitset.

Constraints:

    1 <= size <= 10^5
    0 <= idx <= size - 1
    At most 105 calls will be made in total to fix, unfix, flip, all, one, count, and toString.
    At least one call will be made to all, one, count, or toString.
    At most 5 calls will be made to toString.
*/

pub struct Bitset {
    bits: Vec<bool>,
    count: usize,
    flipped: bool,
}

impl Bitset {
    pub fn new(size: i32) -> Self {
        Self {
            bits: vec![false; size as usize],
            count: 0,
            flipped: false,
        }
    }

    pub fn fix(&mut self, idx: i32) {
        if self.bits[idx as usize] == false ^ self.flipped {
            self.bits[idx as usize] = true ^ self.flipped;
            if self.flipped {
                self.count -= 1;
            } else {
                self.count += 1;
            }
        }
    }

    pub fn unfix(&mut self, idx: i32) {
        if self.bits[idx as usize] == true ^ self.flipped {
            self.bits[idx as usize] = false ^ self.flipped;
            if self.flipped {
                self.count += 1;
            } else {
                self.count -= 1;
            }
        }
    }

    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }

    pub fn all(&self) -> bool {
        (self.count == self.bits.len() && !self.flipped) || (self.count == 0 && self.flipped)
    }

    pub fn one(&self) -> bool {
        if self.flipped {
            self.count < self.bits.len()
        } else {
            self.count > 0
        }
    }

    pub fn count(&self) -> i32 {
        if self.flipped {
            self.bits.len() as i32 - self.count as i32
        } else {
            self.count as i32
        }
    }

    // pub fn to_string(&self) -> String {
    //     let mut s: String = String::with_capacity(self.bits.len());
    //     for bit in self.bits.iter() {
    //         s.push(if *bit ^ self.flipped { '1' } else { '0' });
    //     }
    //     s
    // }
}

impl std::fmt::Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for bit in self.bits.iter() {
            write!(f, "{}", if *bit ^ self.flipped { '1' } else { '0' })?;
        }
        Ok(())
    }
}

#[test]
fn test() {
    let mut bs = Bitset::new(5);
    bs.fix(3);
    bs.fix(1);
    bs.flip();
    assert!(!bs.all());
    bs.unfix(0);
    bs.flip();
    assert!(bs.one());
    bs.unfix(0);
    assert_eq!(bs.count(), 2);
    assert_eq!(bs.to_string(), "01010");
}
