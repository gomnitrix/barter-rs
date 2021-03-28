use crate::error::BarterError;
use crate::data::market::{MarketEvent, Bar};

use std::vec::IntoIter;
use uuid::Uuid;
use serde::Deserialize;
use chrono::Utc;
use crate::data::error::DataError;
use crate::data::error::DataError::{DataIteratorEmpty, BuilderIncomplete};

/// Communicates if a process should continue.
pub trait Continuer {
    fn should_continue(&self) -> bool;
}

/// Generates the latest MarketEvent, acting as the system heartbeat.
pub trait MarketGenerator {
    fn generate_market(&mut self) -> Result<MarketEvent, DataError>;
}

/// Configuration for constructing a HistoricDataHandler via the new() constructor method.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub data_directory: String,
    pub file_type: String,
    pub exchange: String,
    pub symbol: String,
    pub timeframe: String,
}

/// MarketEvent data handler that implements Continuer & MarketGenerator. Simulates a live market
/// feed via drip feeding historical data files.
pub struct HistoricDataHandler {
    exchange: String,
    symbol: String,
    all_symbol_data: IntoIter<Bar>,
}

impl Continuer for HistoricDataHandler {
    fn should_continue(&self) -> bool {
        self.all_symbol_data.len() != 0
    }
}

impl MarketGenerator for HistoricDataHandler {
    fn generate_market(&mut self) -> Result<MarketEvent, DataError> {
        match self.all_symbol_data.next() {
            None => Err(DataIteratorEmpty()),
            Some(bar) => Ok(MarketEvent::builder()
                .trace_id(Uuid::new_v4())
                .timestamp(Utc::now())
                .exchange(self.exchange.clone())
                .symbol(self.symbol.clone())
                .bar(bar)
                .build()?),
        }
    }
}

impl HistoricDataHandler {
    /// Constructs a new HistoricDataHandler component using the provided Configuration struct.
    pub fn new(cfg: &Config) -> Self {
        let file_path = build_symbol_data_file_path(&cfg);

        let bar_iter = load_csv_bars(file_path)
            .expect("Failed to load_csv_bars from provided filepath")
            .into_iter();

        HistoricDataHandler::builder()
            .exchange(cfg.exchange.clone())
            .symbol(cfg.symbol.clone())
            .all_symbol_data(bar_iter)
            .build()
            .expect("Failed to build HistoricDataHandler")
    }

    /// Returns a HistoricDataHandlerBuilder instance.
    pub fn builder() -> HistoricDataHandlerBuilder {
        HistoricDataHandlerBuilder::new()
    }
}

/// Builds a URI using the provided Configuration struct that points to a symbol data file.
pub fn build_symbol_data_file_path(config: &Config) -> String {
    format!("{}{}_{}.{}", config.data_directory, config.symbol, config.timeframe, config.file_type)
}

/// Loads symbol data into a vector of Bars from the provided file URI.
pub fn load_csv_bars(file_path: String) -> Result<Vec<Bar>, csv::Error> {
    // Read the file
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path(file_path)?;

    // Iterate through the lines & add the deserialised Bar struct to Vec<Bar>
    let deserialised_records_iter = reader.deserialize();
    let mut bar_data = match deserialised_records_iter.size_hint().1 {
        None => Vec::new(),
        Some(approx_iter_size) => Vec::with_capacity(approx_iter_size),
    };
    for result_bar in deserialised_records_iter {
        bar_data.push(result_bar?);
    }

    Ok(bar_data)
}

/// Builder to construct HistoricDataHandler instances.
pub struct HistoricDataHandlerBuilder {
    exchange: Option<String>,
    symbol: Option<String>,
    all_symbol_data: Option<IntoIter<Bar>>,
}

impl HistoricDataHandlerBuilder {
    pub fn new() -> Self {
        Self {
            exchange: None,
            symbol: None,
            all_symbol_data: None,
        }
    }

    pub fn symbol(mut self, value: String) -> Self {
        self.symbol = Some(value);
        self
    }

    pub fn exchange(mut self, value: String) -> Self {
        self.exchange = Some(value);
        self
    }

    pub fn all_symbol_data(mut self, value: IntoIter<Bar>) -> Self {
        self.all_symbol_data = Some(value);
        self
    }

    pub fn build(self) -> Result<HistoricDataHandler, DataError> {
        if let (Some(exchange), Some(symbol), Some(all_symbol_data)) =
        (self.exchange, self.symbol, self.all_symbol_data) {
            Ok(HistoricDataHandler {
                exchange,
                symbol,
                all_symbol_data,
            })
        } else {
            Err(BuilderIncomplete())
        }
    }
}