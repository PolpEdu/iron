mod affinity;
pub mod events;
mod global_state;
mod tokens;
pub mod ui_events;
pub use foundry_utils::types::{ToAlloy, ToEthers};

pub use affinity::Affinity;
pub use alloy_primitives::{address, Address, B256, U256, U64};
pub use ethers::abi::Abi;
pub use ethers::types::Bytes;
pub use events::Event;
pub use global_state::GlobalState;
pub use tokens::{
    Erc721Collection, Erc721Token, Erc721TokenData, Erc721TokenDetails, TokenBalance, TokenMetadata,
};
pub use ui_events::UINotify;

pub type Json = serde_json::Value;

#[derive(Debug, Default)]
pub struct SyncUpdates {
    pub events: Option<Vec<Event>>,
    pub erc20_balances: Option<Vec<(Address, U256)>>,
    pub native_balance: Option<U256>,
    pub tip: Option<u64>,
}
