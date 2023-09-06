pub mod async_data { 

use std::error::Error;
use std::time::UNIX_EPOCH;
use yahoo_finance_api as yahoo;
use cli_candlestick_chart::{Candle, Chart};

use tokio::time::Duration; // Import interval from tokioI 
pub async fn fetch_and_append_data(ticker: &str) -> Result<(), Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new();

    // Fetch historical data for the initial chart
    let response = tokio_test::block_on(provider.get_quote_range(ticker, "1min", "1d"))?;
    let quotes = response.quotes()?;

    // Convert Yahoo Finance quote data to candlestick data
    let mut candles = quotes
        .iter()
        .map(|quote| {
            let _timestamp = UNIX_EPOCH + time::Duration::seconds(quote.timestamp as i64);
            let open = quote.open as f64;
            let high = quote.high as f64;
            let low = quote.low as f64;
            let close = quote.close as f64;

            Candle::new(open, high, low, close, Some(quote.volume as f64), Some(quote.timestamp as i64))
        })
        .collect::<Vec<Candle>>();

    // Create the initial chart
    let mut chart = Chart::new(&candles);
    chart.set_name(format!("{} Candlestick Chart", ticker));
    chart.set_bear_color(0, 0, 139); // Dark Blue: RGB(0, 0, 139)
    chart.set_bull_color(255, 255, 255); // White: RGB(255, 255, 255)
    chart.draw();

    // Start fetching and appending new data asynchronously in 1-minute intervals
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Use tokio's interval
 
    while let Some(_) = tokio::time::timeout(Duration::from_secs(60), interval.tick()).await.ok() {
        // Your code here
    
        
       
    
        // Fetch the latest quote
        let latest_response = tokio_test::block_on(provider.get_latest_quotes(ticker, "1d"))?;
        
        // Extract data from the latest quote within this limited scope
        if let Ok(latest_quote) = latest_response.last_quote() {
            let _timestamp = UNIX_EPOCH + time::Duration::seconds(latest_quote.timestamp as i64);
            let open = latest_quote.open as f64;
            let high = latest_quote.high as f64;
            let low = latest_quote.low as f64;
            let close = latest_quote.close as f64;

            // Append the new data point to the chart
            candles.push(Candle::new(open, high, low, close, Some(latest_quote.volume as f64), Some(latest_quote.timestamp as i64)));

            // Create and update the chart
            let mut chart = Chart::new(&candles);
            chart.set_name(format!("{} Candlestick Chart", ticker));
            chart.set_bear_color(0, 0, 139); // Dark Blue: RGB(0, 0, 139)
            chart.set_bull_color(255, 255, 255); // White: RGB(255, 255, 255)
            chart.draw();

            // Print the latest quote
            let time = time::Duration::seconds(latest_quote.timestamp as i64);
            println!("At {} quote price of {} was {}", time, ticker, close);
        }
    }

    Ok(())
} 
}
