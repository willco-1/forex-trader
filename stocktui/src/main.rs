use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use ratatui::{
    event::{self, Event, KeyCode, KeyModifiers},
    prelude::*,
    terminal::Terminal,
};

mod stock_data;
mod stock_data_widget;

use stock_data::{StockData, StockApiResponse};
use stock_data_widget::StockDataWidget;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the terminal
    let mut terminal = Terminal::new()?;

    // Create a new StockData instance
    let stock_data = Arc::new(StockData::new("AAPL")); // Replace with desired stock symbol

    // Clone the Arc for use in the closure
    let cloned_stock_data = Arc::clone(&stock_data);

    // Set up the event handler
    terminal.events_loop.on_event(move |event| {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            state,
        }) = event
        {
            if state == KeyState::Pressed {
                match (code, modifiers) {
                    (KeyCode::Char('q'), _) => {
                        terminal.set_should_exit(true);
                    }
                    _ => {}
                }
            }
        }
    });

    // Launch continuous data fetching and rendering
    tokio::spawn(async move {
        cloned_stock_data.continuously_fetch_data().await;
    });

    // Main event loop
    while terminal.is_running() {
        terminal.clear()?;

        // Render stock data using the TUI
        terminal.render(|frame| {
            StockDataWidget::new(&stock_data).render(frame);
        })?;

        // Sleep for a while before updating again
        tokio::time::sleep(Duration::from_secs(60)).await;
    }

    Ok(())
}
