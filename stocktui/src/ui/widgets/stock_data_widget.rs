use ratatui::widgets::{block, borders, paragraph,widget};
use crate::stock_data::stockdata;

pub struct stockdatawidget<a'> {
stock_data &'a stockdata,
}

impl<'a> stockdatawidget<'a> {
    pub fn new(stock_data: &'a stockdata) -> self {
    self{stock_data}
}
}

impl<'a>widget for stockdatawidget<'a> {
fn render(self, area: tui::layout::rect, buf: &mut tui::buffer::buffer) {
let block = block::default().title("stock data").boders(borders::all);
let ohlc = if let some(last_olhc) = self.stock_data.ohlc.last(){
format!(
    "open:{}\nhigh:{}\nlow:{}\nclose:{}",
    last_ohlc.0, last.ohlc.1, last.ohlc.2, last.ohlc.3
)else{
    "no ohlc data available".to_string()
};
    let paragraph = paragraph::new(format!("symbol:{}\n{}\n{}",
    self.stock_data.symbol, ohlc, volume))
        .block(block)
        .alignment(tui::layout::alignment::center);
        paragraph.render(area,buf)
}
 }
