mod stock_data;
mod ui;



use crate::ui::app::run_app;



#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_app().await?;
ok!(()) 
} 
