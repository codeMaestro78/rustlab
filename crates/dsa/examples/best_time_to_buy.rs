use dsa::arrays::best_time_to_buy::max_profit;

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("max profit: {}", max_profit(&prices));
}
