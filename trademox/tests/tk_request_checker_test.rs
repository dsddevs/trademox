use dotenv::dotenv;
use env::var;
use std::env;
use tradebox::requests::tk_request::TickerRequest;
use tradebox::requests::tk_request_checker::{get_default_or_end, get_default_or_start};

#[test]
fn validates_date_format_env() {
    dotenv().ok();
    let def_ticker = var("DEFAULT_TICKER").expect("Invalid ticker");
    let def_start = var("DEFAULT_START").expect("Invalid start");

    let req = TickerRequest {
        ticker: def_ticker,
        start: def_start,
        end: "bad-date".into(),
    };

    assert!(get_default_or_start(&req).is_ok());
    assert!(get_default_or_end(&req).is_err());
}
