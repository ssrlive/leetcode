#![allow(dead_code)]

/*

// 1656. Design an Ordered Stream
// https://leetcode.com/problems/design-an-ordered-stream/
// https://leetcode.cn/problems/design-an-ordered-stream/
//
// Easy
//
// There is a stream of n (idKey, value) pairs arriving in an arbitrary order, where idKey is an integer between 1 and n and value is a string. No two pairs have the same id.

Design a stream that returns the values in increasing order of their IDs by returning a chunk (list) of values after each insertion. The concatenation of all the chunks should result in a list of the sorted values.

Implement the OrderedStream class:

    OrderedStream(int n) Constructs the stream to take n values.
    String[] insert(int idKey, String value) Inserts the pair (idKey, value) into the stream, then returns the largest possible chunk of currently inserted values that appear next in the order.

Example:

Input
["OrderedStream", "insert", "insert", "insert", "insert", "insert"]
[[5], [3, "ccccc"], [1, "aaaaa"], [2, "bbbbb"], [5, "eeeee"], [4, "ddddd"]]
Output
[null, [], ["aaaaa"], ["bbbbb", "ccccc"], [], ["ddddd", "eeeee"]]

Explanation
// Note that the values ordered by ID is ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"].
OrderedStream os = new OrderedStream(5);
os.insert(3, "ccccc"); // Inserts (3, "ccccc"), returns [].
os.insert(1, "aaaaa"); // Inserts (1, "aaaaa"), returns ["aaaaa"].
os.insert(2, "bbbbb"); // Inserts (2, "bbbbb"), returns ["bbbbb", "ccccc"].
os.insert(5, "eeeee"); // Inserts (5, "eeeee"), returns [].
os.insert(4, "ddddd"); // Inserts (4, "ddddd"), returns ["ddddd", "eeeee"].
// Concatentating all the chunks returned:
// [] + ["aaaaa"] + ["bbbbb", "ccccc"] + [] + ["ddddd", "eeeee"] = ["aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee"]
// The resulting order is the same as the order above.

Constraints:

    1 <= n <= 1000
    1 <= id <= n
    value.length == 5
    value consists only of lowercase letters.
    Each call to insert will have a unique id.
    Exactly n calls will be made to insert.
*/

struct OrderedStream {
    n: usize,
    values: Vec<String>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            n: n as usize,
            values: vec![String::new(); n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let id = id as usize;
        self.values[id - 1] = value;
        let mut res = vec![];
        while self.ptr < self.n && !self.values[self.ptr].is_empty() {
            res.push(self.values[self.ptr].clone());
            self.ptr += 1;
        }
        res
    }
}

#[test]
fn test() {
    let mut os = OrderedStream::new(5);
    assert_eq!(os.insert(3, "ccccc".to_string()), Vec::<String>::new());
    assert_eq!(os.insert(1, "aaaaa".to_string()), vec!["aaaaa".to_string()]);
    assert_eq!(os.insert(2, "bbbbb".to_string()), vec!["bbbbb".to_string(), "ccccc".to_string()]);
    assert_eq!(os.insert(5, "eeeee".to_string()), Vec::<String>::new());
    assert_eq!(os.insert(4, "ddddd".to_string()), vec!["ddddd".to_string(), "eeeee".to_string()]);
}
