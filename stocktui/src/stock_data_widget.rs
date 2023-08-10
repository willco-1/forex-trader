pub mod stock_data_widget { 
    use ratatui::widgets::{Block, Borders, Paragraph, Widget};
    use stock_data::StockData;

pub struct StockDataWidget<'a> {
    stock_data: &'a StockData,
}

impl<'a> StockDataWidget<'a> {
    pub fn new(stock_data: &'a StockData) -> Self {
        Self { stock_data }
    }
}

impl<'a> Widget for StockDataWidget<'a> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let block = Block::default().title("Stock Data").borders(Borders::ALL);

        let ohlc = if let Some(last_ohlc) = self.stock_data.ohlc.last() {
            format!(
                "Open: {}\nHigh: {}\nLow: {}\nClose: {}",
                last_ohlc.0, last_ohlc.1, last_ohlc.2, last_ohlc.3
            )
        } else {
            "No OHLC data available".to_string()
        };

        let volume = if let Some(last_volume) = self.stock_data.volume.last() {
            format!("Volume: {}", last_volume)
        } else {
            "No volume data available".to_string()
        };

        let paragraph = Paragraph::new(format!(
            "Symbol: {}\n{}\n{}",
            self.stock_data.symbol, ohlc, volume
        ))
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);

        paragraph.render(area, buf);
    }
}
}