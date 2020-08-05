use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use super::stock::Stock;


pub fn file_to_stocks(filename: &str) -> Vec<Stock> {
    // Read file of stock OHLC data and return a Vector of Stocks
    let mut v = vec![];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        v.push(parse_str_to_stock(&line));
    }
    return v
}


fn parse_str_to_stock(stock_data: &str) -> Stock {
    // Parse a String of OHLC stock data into a Stock struct
    let vals: Vec<&str> = stock_data.split(", ").collect();
    return Stock{
        name: String::from(vals[0]),
        price_open: vals[1].parse::<f64>().unwrap(),
        price_high: vals[2].parse::<f64>().unwrap(),
        price_low: vals[3].parse::<f64>().unwrap(),
        price_close: vals[4].parse::<f64>().unwrap()
    };
    
}
