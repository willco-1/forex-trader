pub mod stock_data {
use reqwest;
use serde::Deserialize;
use tokio;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use crate::stock_data_widget::Backend;

// Define the Backend trait
pub trait Backend {
    fn clear(&self);
    fn move_to(&self, x: u16, y: u16);
    // Add other necessary methods here
}

// Implement Backend for CrosstermBackend
pub struct CrosstermBackend;

impl Backend for CrosstermBackend {
    fn clear(&self) {
        execute!(std::io::stdout(), terminal::Clear(ClearType::All)).unwrap();
    }

    fn move_to(&self, x: u16, y: u16) {
        execute!(std::io::stdout(), cursor::MoveTo(x, y)).unwrap();
    }

    // Implement other methods here
}

// Define the StockData struct
pub struct StockData {
    pub symbol: String,
    pub ohlc: Vec<(f64, f64, f64, f64)>,
    pub volume: Vec<f64>,
    pub backend: Box<dyn Backend>,
}

impl StockData {
    // Constructor to create StockData with backend
    pub fn with_backend(symbol: &str, backend: Box<dyn Backend>) -> Self {
        Self {
            symbol: symbol.to_string(),
            ohlc: Vec::new(),
            volume: Vec::new(),
            backend,
        }
    }

    // Fetch data from the API and populate ohlc and volume
    pub async fn get_data(&mut self) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://query1.finance.yahoo.com/v7/finance/chart/{}?range=1d&interval=1m",
            self.symbol
        );

        let response = reqwest::get(&url).await?;
        let api_response: StockApiResponse = response.json().await?;

        self.ohlc = vec![(
            api_response.open,
            api_response.high,
            api_response.low,
            api_response.close,
        )];
        self.volume = vec![api_response.volume];

        Ok(())
    }

    // Render candlesticks and volume charts using crossterm
    pub fn render_charts(&self) {
        self.backend.clear();
        self.backend.move_to(0, 0);

        // Render candlesticks and volume charts using crossterm
        self.render_candlesticks();
        self.render_volume();
    }

    // Implement rendering candlesticks using crossterm
    fn render_candlesticks(&self) {
        // Implement rendering candlesticks logic here
    }

    // Implement rendering volume chart using crossterm
    fn render_volume(&self) {
        // Implement rendering volume chart logic here
    }
}

#[derive(Debug, Deserialize)]
struct StockApiResponse {
    open: f64,
    high: f64,
    close: f64,
    low: f64,
    volume: f64,
}

// Function to fetch data continuously and render charts
pub async fn continuously_fetch_and_render_data(symbol: &str, backend: Box<dyn Backend>) {
    let mut stock_data = StockData::with_backend(symbol, backend);

    loop {
        if let Err(_) = stock_data.get_data().await {
            // Handle the error appropriately
        }

        stock_data.render_charts();

        // Wait for a duration before fetching data again
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
impl<'a> Backend for &'a StockData {
    fn clear(&self) {
        self.backend.clear();
    }

    fn move_to(&self, x: u16, y: u16) {
        self.backend.move_to(x, y);
    }
}
pub struct StockDataWithBackend {
    pub stock_data: StockData,
    pub backend: CrosstermBackend,
}
impl Backend for StockDataWithBackend {
    fn clear(&self) {
        self.backend.clear();
    }

    fn move_to(&self, x: u16, y: u16) {
        self.backend.move_to(x, y);
    }

    // Implement other necessary methods here
}

}