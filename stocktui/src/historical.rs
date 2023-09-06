pub mod historical { 
use std::error::Error;

use std::time::UNIX_EPOCH;
use yahoo_finance_api as yahoo;
use cli_candlestick_chart::Candle;
use tokio_test;
pub fn fetch_data(ticker: &str, timeframe: &str) ->  Vec<Candle> {
      // Parse command-line arguments for ticker symbol and timeframe
    
    
    // Fetch Yahoo Finance quote data (replace 'AAPL' and interval with your desired symbol and interval)
    let provider = yahoo::YahooConnector::new();
    let response = tokio_test::block_on(provider.get_quote_range(ticker, "1d", timeframe)).unwrap();
    let quotes = response.quotes().unwrap();

    // Convert Yahoo Finance quote data to candlestick data
    let candles = quotes
        .iter()
        .map(|quote| {
            let _timestamp = UNIX_EPOCH + time::Duration::seconds(quote.timestamp as i64);
            let open = quote.open as f64;
            let high = quote.high as f64;
            let low = quote.low as f64;
            let close = quote.close as f64;
            // Adjust this as needed

            Candle::new(open, high, low, close, Some(quote.volume as f64), Some(quote.timestamp as i64))
        })
        .collect::<Vec<Candle>>();
        candles
    // Create and display the chart
    
}
}