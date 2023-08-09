use reqwest;
use tokio;


use serde::Deserialize; // Add this import

#[derive(Debug, Deserialize)]
pub struct StockApiResponse {
    open: f64,
    high: f64,
    close: f64,
    low: f64,
    volume: f64,
}

pub struct StockData {
    pub symbol: String,
   pub ohlc: Vec<(f64, f64, f64, f64)>,
    pub volume: Vec<f64>,
}

impl StockData {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            ohlc: Vec::new(),
            volume: Vec::new(),
        }
    }

    pub async fn fetch_data(symbol: &str) -> Result<StockData, reqwest::Error> {
        let url = format!("https://query1.finance.yahoo.com/v7/finance/chart/{}?range=1d&interval=1m", symbol);

        let response = reqwest::get(&url).await?;
        let api_response: StockApiResponse = response.json().await?;

        let ohlc = vec![(api_response.open, api_response.high, api_response.low, api_response.close)];
        let volume = vec![api_response.volume];

        Ok(StockData {
            symbol: symbol.to_string(),
            ohlc,
            volume,
        })
    }

    pub async fn continuously_fetch_data(symbol: &str) -> ! {
        let mut ohlc = Vec::new();
        let mut volume = Vec::new();

        loop {
            match Self::fetch_data(symbol).await {
                Ok(data) => {
                    let new_ohlc = vec![(data.ohlc[0].0, data.ohlc[0].1, data.ohlc[0].2, data.ohlc[0].3)];
                    let new_volume = vec![data.volume[0]];

                    let cloned_ohlc = new_ohlc.clone();
                    tokio::spawn(async move {
                        Self::render_candlesticks(&cloned_ohlc).await;
                    });

                    ohlc = new_ohlc;
                    volume = new_volume;
                }
                Err(_) => {
                    // Handle the error appropriately
                }
            }

            {
               
            }
        }
    }

    async fn render_candlesticks(_data: &[(f64, f64, f64, f64)]) {
        // Implement rendering candlesticks using the data
        // Example: Render candlesticks chart
        // ...
    }


}

    
