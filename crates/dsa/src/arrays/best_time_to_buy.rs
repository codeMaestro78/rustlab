/// Returns the maximum profit from a single buy-sell transaction.
/// Returns 0 if no profit is possible.
pub fn max_profit(_prices: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for &price in _prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn basic() {
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(&[]), 0);
    }
}
