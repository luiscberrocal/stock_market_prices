fn main() {
    println!("Stock price");
    let stocks = [310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
    println!("Stocks {}", stocks.len());
    let mut idx = 1;
    for stock in stocks {
        if idx != stocks.len() {
            let dif = stocks[idx] - stock;
            println!("Stock {}: ${} ${} {}", idx, stock, stocks[idx], dif);
        }
        idx += 1;
    }
}
