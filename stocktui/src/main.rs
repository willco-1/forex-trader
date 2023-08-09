mod stock_data;
mod ui;

``

use crate::ui::app::run_app;



#[tokio::main]

async fn main() -> result<(),box<dyn std::error::error>> {
    if let err(e) = run_app().await{
        eprintln("error {}",e);
    }
}
ok(())
