use crate::objects::klines::Kline;
use chrono::Utc;

mod moving_average_tests;
mod stochastic_tests;

fn kline_dummy_new(open: f64, high: f64, low: f64, close: f64) -> Kline {
    Kline {
        open_time: Utc::now(),
        open: open,
        high: high,
        low: low,
        close: close,
        volume: 0.0,
        close_time: Utc::now(),
        quote_asset_volume: 0.0,
        number_of_trades: 0,
        taker_buy_base_asset_volume: 0.0,
        taker_buy_quote_asset_volume: 0.0,
    }
}
