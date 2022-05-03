use clap::Parser;

/// Commandline's arguments type
#[derive(Debug, Parser)]
#[clap(author="Wasin Thonkaew (wasin@wasin.io)")]
#[clap(name="contractdiff")]
#[clap(about="Commandline program to output color-diff of contract code between two input contract addresses.
Suitable to use against non-verified contract code for analysis.")]
pub struct CommandlineArgs {
    /// First contract address acting as a base to diff against another address
    #[clap(index=1, required=true)]
    pub address1: String,

    /// Second contract address to diff against
    #[clap(index=2, required=true)]
    pub address2: String,

    /// Which chain to work with.
    #[clap(long="chain", short='c', required=true, multiple_values=false, possible_values=["bsc", "ethereum", "polygon"], ignore_case=true)]
    pub chain: String,
}

/// Chain type
/// Emulate the same name of type which provided by evmscan crate, but to avoid
/// having to add it as a dependency thus pull down other things we don't need.
/// So we define the same name type here.
pub enum ChainType {
	/// BSC - Binance Smart Chain
    BSC,
	/// Ethereum
    Ethereum,
	/// Polygon
    Polygon,
}
