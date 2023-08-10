use crate::stock_data_widget::stock_data_widget::StockDataWidget;;
mod stock_data_widget;
use crate::stock_data_;
// ...

#[tokio::main]
async fn main() {
    // Create a new StockData instance
    let symbol = "AAPL"; // Replace with the desired stock symbol
    let stock_data = StockData::new(&symbol).await.unwrap();

    // Create a new CrosstermBackend instance
    let backend = CrosstermBackend;

    // Create a new StockDataWithBackend instance
    let stock_data_with_backend = StockDataWithBackend {
        stock_data,
        backend,
    };

    // Create a new Ratatui terminal
    let mut terminal = ratatui::Terminal::new(stock_data_with_backend).unwrap();

    // ...

    // Enter the rendering loop
    loop {
        // Clear the terminal and render the StockDataWidget
        terminal.clear().unwrap();
        terminal.draw(|f| {
            stock_data_widget.clear_and_render(f.size(), f.buf);
        }).unwrap();

        // Fetch data and render every 60 seconds
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
