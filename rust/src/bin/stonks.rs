// max profit function given a vector of prices at i-index of days,
// finds out the max differential increase from day i to day j
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    // keep track of the min price seen so far because we want to buy at the
    // lowest price
    let mut min_price = prices[0];
    // iterate through the prices and find the max profit
    // by differential of the min price seen so far
    for i in 1..prices.len() {
        if prices[i] < min_price {
            min_price = prices[i];
        } else if prices[i] - min_price > max_profit {
            max_profit = prices[i] - min_price;
        }
    }
    max_profit
}
pub fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let max_profit = max_profit(prices);
    println!("Max profit is {}", max_profit);
}
