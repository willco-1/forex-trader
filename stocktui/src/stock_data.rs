use reqwest;
use tokio;


 struct api_response { open: f64
                       high: f64
                       close: f64
                       low: f64
                       volume: f64

}

pub struct stock_data { symbol: string
                        ohlc: vec<(f64, f64, f64, f64)>
                        volume: vec<f64>
}

impl stockdata {
pub fn new(symbol:&str)-> self {
self {
symbol: symbol.to_string(),
    ohlc: new::vec(),
    volume: vec::new(),
}
}
pub async fn fetch_data(symbol: &str) -> result<stockdata, reqwest::error> {
    let url = format("https://query1.finance.yahoo.com/v7/finance/chart/{}?range=1d&interval=1m",symbol);
    let response = reqwest::get(&url).await?,
    let api_response:api_response = response.json().await?;
    Ok(stock_data)
}
pub async fn continiously_fetch_data(symbol: &str) {
    let mut ohlc = vec::new();
    let mut volume = vec::new()
    loop {
    match self::fetch_data(symbol).await {
        Ok(data) => {
            ohlc.push((data.ohlc[0].0, data.ohlc[0].1,data.ohlc[0].2, data.ohlc[0].3));
            volume.push(data.volume[0]);
            task::spawn(slef::render_candlesticks(&ohlc));
            ohlc.clear();
            volume.clear();


        }
 err(e) => eprintln!("error fetching {}",e),

        i    }
            sleep(duration::from_secs(60).await;)
        }
    }
    async fn render_candlesticks(dara: &[(f64,f64,f64,f64)]){
        // UNIMPL
    }


 }
