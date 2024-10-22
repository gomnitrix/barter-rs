use super::Binance;
use crate::{instrument::MarketInstrumentData, subscription::Subscription, Identifier};
use barter_instrument::{asset::symbol::Symbol, instrument::Instrument, Keyed};
use serde::{Deserialize, Serialize};
use smol_str::{format_smolstr, SmolStr, StrExt};

/// Type that defines how to translate a Barter [`Subscription`] into a [`Binance`]
/// market that can be subscribed to.
///
/// See docs: <https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams>
/// See docs: <https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct BinanceMarket(pub SmolStr);

impl<Server, Kind> Identifier<BinanceMarket> for Subscription<Binance<Server>, Instrument, Kind> {
    fn id(&self) -> BinanceMarket {
        binance_market(&self.instrument.base, &self.instrument.quote)
    }
}

impl<Server, InstrumentKey, Kind> Identifier<BinanceMarket>
    for Subscription<Binance<Server>, Keyed<InstrumentKey, Instrument>, Kind>
{
    fn id(&self) -> BinanceMarket {
        binance_market(
            &self.instrument.as_ref().base,
            &self.instrument.as_ref().quote,
        )
    }
}

impl<Server, Kind> Identifier<BinanceMarket>
    for Subscription<Binance<Server>, MarketInstrumentData, Kind>
{
    fn id(&self) -> BinanceMarket {
        BinanceMarket(self.instrument.name_exchange.clone())
    }
}

impl AsRef<str> for BinanceMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub(in crate::exchange::binance) fn binance_market(base: &Symbol, quote: &Symbol) -> BinanceMarket {
    // Notes:
    // - Must be lowercase when subscribing (transformed to lowercase by Binance fn requests).
    // - Must be uppercase since Binance sends message with uppercase MARKET (eg/ BTCUSDT).
    BinanceMarket(format_smolstr!("{base}{quote}").to_uppercase_smolstr())
}
