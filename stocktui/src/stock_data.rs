use reqwest;
use tokio;


 struct apiResponse { open: f64,
                       high: f64,
                       close: f64,
                       low: f64,
                       volume: f64

}

pub struct StockData { symbol: String,
                        ohlc: Vec<(f64, f64, f64, f64)>,
                        volume: Vec<f64>,
}

impl StockData {
pub fn new(symbol:&str)-> Self {
Self {
symbol: symbol.to_string(),
    ohlc: Self::new::vec(),
    volume: Vec::new(),
}
}
pub async fn fetch_data(symbol: &str) -> <Result<StockData, reqwest::Error> {
    let url: String = format!("https://query1.finance.yahoo.com/v7/finance/chart/{}?range=1d&interval=1m",symbol);
    let response = reqwest::get(&url).await?;
    let api_response:api_response = response.json().await?;
    Ok(Ok(StockData))
}
pub async fn continiously_fetch_data(symbol: &str) {
    let mut ohlc = Vec::new();
    let mut volume = Vec::new();
    loop {
    match self::fetch_data(symbol).await {
        Ok(data) => {
            ohlc.push((data.ohlc[0].0, data.ohlc[0].1,data.ohlc[0].2, data.ohlc[0].3));
            volume.push(data.volume[0]);
            task::spawn(self::render_candlesticks(&ohlc));
            ohlc.clear();
            volume.clear();


        }
 Err(e) => eprintln!("error fetching {}",e),

        i    }
            tokio::time::sleep(tokio::time::Duration::from_secs(60).await);
        }
    }
    async fn render_candlesticks(dara: &[(f64,f64,f64,f64)]){
        // UNIMPL
    }


 }

