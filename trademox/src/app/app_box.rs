use crate::creator::tk_creator::TickerBox;
use crate::data::yfinance_data::YFinanceData;

#[derive(Debug, Clone)]
pub struct AppBox {
    pub yfinance_data: YFinanceData,
    pub ticker_box: TickerBox,
}

pub fn build_app() -> AppBox {
    AppBox {
        yfinance_data: YFinanceData::new(),
        ticker_box: TickerBox::new(),
    }
}
