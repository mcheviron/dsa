pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price = prices[0];
    for i in 1..prices.len() {
        max_profit = max_profit.max(prices[i] - min_price);
        min_price = min_price.min(prices[i]);
    }
    max_profit
}
