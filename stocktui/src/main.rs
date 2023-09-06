

use std::env;
use std::error::Error;
pub mod async_data;
pub mod historical;
use cli_candlestick_chart::Chart;
#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments for ticker symbol and timeframe
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <ticker> <timeframe>", args[0]);
        std::process::exit(1);
    }
    let ticker = &args[1];
    let timeframe = &args[2];

    // Fetch historical data and get the candles
    let candles = historical::historical::fetch_data(ticker, timeframe);

    // Create and display the chart
    let mut chart = Chart::new(&candles);
    chart.set_bear_color(0, 0, 139); // Dark Blue: RGB(0, 0, 139)
    chart.set_bull_color(255, 255, 255); // White
    chart.set_name(String::from(ticker)); // Set your desired chart name
    chart.set_vol_bear_color(0, 0, 139);
    chart.set_vol_bull_color(255, 255, 255);
    chart.set_volume_pane_height(4);
    chart.draw();

    Ok(())
}