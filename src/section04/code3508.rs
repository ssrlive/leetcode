#![allow(dead_code)]

// 3508. Implement Router
// https://leetcode.com/problems/implement-router/
// https://leetcode.cn/problems/implement-router/
//
// Medium
//
// Design a data structure that can efficiently manage data packets in a network router. Each data packet consists of the following attributes:
//
//     source: A unique identifier for the machine that generated the packet.
//     destination: A unique identifier for the target machine.
//     timestamp: The time at which the packet arrived at the router.
//
// Implement the Router class:
//
// Router(int memoryLimit): Initializes the Router object with a fixed memory limit.
//
//     memoryLimit is the maximum number of packets the router can store at any given time.
//     If adding a new packet would exceed this limit, the oldest packet must be removed to free up space.
//
// bool addPacket(int source, int destination, int timestamp): Adds a packet with the given attributes to the router.
//
//     A packet is considered a duplicate if another packet with the same source, destination, and timestamp already exists in the router.
//     Return true if the packet is successfully added (i.e., it is not a duplicate); otherwise return false.
//
// int[] forwardPacket(): Forwards the next packet in FIFO (First In First Out) order.
//
//     Remove the packet from storage.
//     Return the packet as an array [source, destination, timestamp].
//     If there are no packets to forward, return an empty array.
//
// int getCount(int destination, int startTime, int endTime):
//
// - Returns the number of packets currently stored in the router (i.e., not yet forwarded) that have
//   the specified destination and have timestamps in the inclusive range [startTime, endTime].
//
// Note that queries for addPacket will be made in increasing order of timestamp.
//
// Example 1:
//
// Input:
// ["Router", "addPacket", "addPacket", "addPacket", "addPacket", "addPacket", "forwardPacket", "addPacket", "getCount"]
// [[3], [1, 4, 90], [2, 5, 90], [1, 4, 90], [3, 5, 95], [4, 5, 105], [], [5, 2, 110], [5, 100, 110]]
//
// Output:
// [null, true, true, false, true, true, [2, 5, 90], true, 1]
//
// Explanation
// Router router = new Router(3); // Initialize Router with memoryLimit of 3.
// router.addPacket(1, 4, 90); // Packet is added. Return True.
// router.addPacket(2, 5, 90); // Packet is added. Return True.
// router.addPacket(1, 4, 90); // This is a duplicate packet. Return False.
// router.addPacket(3, 5, 95); // Packet is added. Return True
// router.addPacket(4, 5, 105); // Packet is added, [1, 4, 90] is removed as number of packets exceeds memoryLimit. Return True.
// router.forwardPacket(); // Return [2, 5, 90] and remove it from router.
// router.addPacket(5, 2, 110); // Packet is added. Return True.
// router.getCount(5, 100, 110); // The only packet with destination 5 and timestamp in the inclusive range [100, 110] is [4, 5, 105]. Return 1.
//
// Example 2:
//
// Input:
// ["Router", "addPacket", "forwardPacket", "forwardPacket"]
// [[2], [7, 4, 90], [], []]
//
// Output:
// [null, true, [7, 4, 90], []]
//
// Explanation
// Router router = new Router(2); // Initialize Router with memoryLimit of 2.
// router.addPacket(7, 4, 90); // Return True.
// router.forwardPacket(); // Return [7, 4, 90].
// router.forwardPacket(); // There are no packets left, return [].
//
// Constraints:
//
//     2 <= memoryLimit <= 10^5
//     1 <= source, destination <= 2 * 10^5
//     1 <= timestamp <= 10^9
//     1 <= startTime <= endTime <= 10^9
//     At most 105 calls will be made to addPacket, forwardPacket, and getCount methods altogether.
//     queries for addPacket will be made in increasing order of timestamp.
//

use std::collections::{BTreeMap, HashMap};

struct Router {
    mpp: BTreeMap<Vec<i32>, i32>,       // to track duplicates
    queue: Vec<Vec<i32>>,               // to store packets in FIFO order
    timestamps: HashMap<i32, Vec<i32>>, // for timestamps tracking
    st: HashMap<i32, i32>,
    max_size: i32, // maxSize allowed
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Router {
            mpp: BTreeMap::new(),
            queue: Vec::new(),
            timestamps: HashMap::new(),
            st: HashMap::new(),
            max_size: memory_limit,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = vec![source, destination, timestamp];
        // checking for duplicate
        if self.mpp.contains_key(&packet) {
            return false;
        }
        if self.queue.len() as i32 == self.max_size {
            // remove the first element if queue is full
            let res = self.queue.remove(0);
            self.mpp.remove(&res);
            let temp = res[1];
            *self.st.entry(temp).or_insert(0) += 1;
        }
        self.queue.push(packet.clone());
        *self.mpp.entry(packet).or_insert(0) += 1;
        self.timestamps.entry(destination).or_default().push(timestamp);
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if self.queue.is_empty() {
            return Vec::new();
        }
        let res = self.queue.remove(0);
        self.mpp.remove(&res);
        let temp = res[1];
        *self.st.entry(temp).or_insert(0) += 1;
        res
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(p) = self.timestamps.get(&destination) {
            let temp = *self.st.get(&destination).unwrap_or(&0);
            let len = p.len() - temp as usize;
            let right = p[temp as usize..].iter().position(|&x| x >= start_time).unwrap_or(len) as i32;
            let left = p[temp as usize..].iter().position(|&x| x > end_time).unwrap_or(len) as i32;
            left - right
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let mut router = Router::new(3);
    assert!(router.add_packet(1, 4, 90));
    assert!(router.add_packet(2, 5, 90));
    assert!(!router.add_packet(1, 4, 90));
    assert!(router.add_packet(3, 5, 95));
    assert!(router.add_packet(4, 5, 105));
    assert_eq!(router.forward_packet(), vec![2, 5, 90]);
    assert!(router.add_packet(5, 2, 110));
    assert_eq!(router.get_count(5, 100, 110), 1);
}
