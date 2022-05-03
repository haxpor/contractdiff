use crate::types::*;

use regex::Regex;
use web3::{
	Web3,
	transports::http::Http,
};

/// RPC endpoint of BSC chain
pub(crate) static BSC_RPC_ENDPOINT: &str = "https://bsc-dataseed.binance.org/";
/// RPC endpoint of Ethereum chain
pub(crate) static ETHEREUM_RPC_ENDPOINT: &str = "https://rpc.ankr.com/eth";
/// RPC endpoint of Polygon chain
pub(crate) static POLYGON_RPC_ENDPOINT: &str = "https://polygon-rpc.com/";

/// Create a web3 instance
pub fn create_web3(chain: ChainType) -> Web3<Http> {
    let rpc_endpoint = match chain {
        ChainType::BSC => BSC_RPC_ENDPOINT,
        ChainType::Ethereum => ETHEREUM_RPC_ENDPOINT,
        ChainType::Polygon => POLYGON_RPC_ENDPOINT,
    };
    let http = Http::new(rpc_endpoint).unwrap();
    Web3::new(http)
}

/// Check whether specified address string is an address.
///
/// This is not full-fledge checking in which it doesn't take into account
/// checking of checksum address.
///
/// # Arguments
/// * `address` - address string to check
pub fn is_address_simplified(address: &str) -> bool {
    let lowercase_address = address.to_lowercase();
    let regex: Regex = Regex::new(r#"^(0x)?[0-9a-f]{40}$"#).unwrap();

    regex.is_match(&lowercase_address)
}
