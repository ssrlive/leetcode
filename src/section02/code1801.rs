#![allow(dead_code)]

/*

// 1801. Number of Orders in the Backlog
// https://leetcode.com/problems/number-of-orders-in-the-backlog/
// https://leetcode.cn/problems/number-of-orders-in-the-backlog/
//
// Medium
//
// You are given a 2D integer array orders, where each orders[i] = [pricei, amounti, orderTypei] denotes that amounti orders have been placed of type orderTypei at the price pricei. The orderTypei is:

    0 if it is a batch of buy orders, or
    1 if it is a batch of sell orders.

Note that orders[i] represents a batch of amounti independent orders with the same price and order type. All orders represented by orders[i] will be placed before all orders represented by orders[i+1] for all valid i.

There is a backlog that consists of orders that have not been executed. The backlog is initially empty. When an order is placed, the following happens:

    If the order is a buy order, you look at the sell order with the smallest price in the backlog. If that sell order's price is smaller than or equal to the current buy order's price, they will match and be executed, and that sell order will be removed from the backlog. Else, the buy order is added to the backlog.
    Vice versa, if the order is a sell order, you look at the buy order with the largest price in the backlog. If that buy order's price is larger than or equal to the current sell order's price, they will match and be executed, and that buy order will be removed from the backlog. Else, the sell order is added to the backlog.

Return the total amount of orders in the backlog after placing all the orders from the input. Since this number can be large, return it modulo 109 + 7.

Example 1:

Input: orders = [[10,5,0],[15,2,1],[25,1,1],[30,4,0]]
Output: 6
Explanation: Here is what happens with the orders:
- 5 orders of type buy with price 10 are placed. There are no sell orders, so the 5 orders are added to the backlog.
- 2 orders of type sell with price 15 are placed. There are no buy orders with prices larger than or equal to 15, so the 2 orders are added to the backlog.
- 1 order of type sell with price 25 is placed. There are no buy orders with prices larger than or equal to 25 in the backlog, so this order is added to the backlog.
- 4 orders of type buy with price 30 are placed. The first 2 orders are matched with the 2 sell orders of the least price, which is 15 and these 2 sell orders are removed from the backlog. The 3rd order is matched with the sell order of the least price, which is 25 and this sell order is removed from the backlog. Then, there are no more sell orders in the backlog, so the 4th order is added to the backlog.
Finally, the backlog has 5 buy orders with price 10, and 1 buy order with price 30. So the total number of orders in the backlog is 6.

Example 2:

Input: orders = [[7,1000000000,1],[15,3,0],[5,999999995,0],[5,1,1]]
Output: 999999984
Explanation: Here is what happens with the orders:
- 109 orders of type sell with price 7 are placed. There are no buy orders, so the 109 orders are added to the backlog.
- 3 orders of type buy with price 15 are placed. They are matched with the 3 sell orders with the least price which is 7, and those 3 sell orders are removed from the backlog.
- 999999995 orders of type buy with price 5 are placed. The least price of a sell order is 7, so the 999999995 orders are added to the backlog.
- 1 order of type sell with price 5 is placed. It is matched with the buy order of the highest price, which is 5, and that buy order is removed from the backlog.
Finally, the backlog has (1000000000-3) sell orders with price 7, and (999999995-1) buy orders with price 5. So the total number of orders = 1999999991, which is equal to 999999984 % (109 + 7).

Constraints:

    1 <= orders.length <= 10^5
    orders[i].length == 3
    1 <= pricei, amounti <= 10^9
    orderTypei is either 0 or 1.
*/

struct Solution;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        #[derive(Eq, Debug, Clone)]
        struct Order {
            price: i32,
            amount: i32,
            order_type: i32,
        }

        impl Order {
            fn new(mut price: i32, amount: i32, order_type: i32) -> Self {
                if order_type == 1 {
                    price *= -1;
                }

                Self {
                    price,
                    amount,
                    order_type,
                }
            }
        }

        impl Ord for Order {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.price.cmp(&other.price)
            }
        }

        impl PartialOrd for Order {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Order {
            fn eq(&self, other: &Self) -> bool {
                self.price == other.price
            }
        }

        let base: i32 = 10;
        let modulo: f64 = (base.pow(9) + 7) as f64;

        let mut buy_backlog: BinaryHeap<Order> = BinaryHeap::new();
        let mut sell_backlog: BinaryHeap<Order> = BinaryHeap::new();

        for order in orders.iter() {
            let mut o = Order::new(order[0], order[1], order[2]);

            while o.amount > 0 {
                if o.order_type == 0 {
                    if !sell_backlog.is_empty() && -sell_backlog.peek().unwrap().price <= o.price {
                        let mut sell_order = sell_backlog.pop().unwrap();
                        let amount = i32::min(sell_order.amount, o.amount);
                        o.amount -= amount;
                        sell_order.amount -= amount;

                        if sell_order.amount > 0 {
                            sell_backlog.push(sell_order);
                        }
                    } else {
                        buy_backlog.push(o.clone());
                        break;
                    }
                } else if !buy_backlog.is_empty() && buy_backlog.peek().unwrap().price >= -o.price {
                    let mut buy_order = buy_backlog.pop().unwrap();
                    let amount = i32::min(buy_order.amount, o.amount);
                    o.amount -= amount;
                    buy_order.amount -= amount;

                    if buy_order.amount > 0 {
                        buy_backlog.push(buy_order);
                    }
                } else {
                    sell_backlog.push(o.clone());
                    break;
                }
            }
        }

        ((buy_backlog.iter().map(|order| order.amount as f64).sum::<f64>()
            + sell_backlog.iter().map(|order| order.amount as f64).sum::<f64>())
            % modulo) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![10, 5, 0], vec![15, 2, 1], vec![25, 1, 1], vec![30, 4, 0]], 6),
        (
            vec![
                vec![7, 1000000000, 1],
                vec![15, 3, 0],
                vec![5, 999999995, 0],
                vec![5, 1, 1],
            ],
            999999984,
        ),
    ];
    for (orders, expected) in cases {
        assert_eq!(Solution::get_number_of_backlog_orders(orders), expected);
    }
}
