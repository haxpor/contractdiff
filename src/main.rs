use similar::ChangeTag;
use console::Style;
use web3::types::Address;
use regex::Regex;

use std::env;

/// Print usage text
fn print_program_usage() {
    eprintln!("Usage: {} <contract-address-1> <contract-address-2>", env!("CARGO_PKG_NAME", "bsccontract-diff"));
}

/// Check whether specified address string is an address.
///
/// This is not full-fledge checking in which it doesn't take into account
/// checking of checksum address.
///
/// # Arguments
/// * `address` - address string to check
fn is_address_simplified(address: &str) -> bool {
    let lowercase_address = address.to_lowercase();
    let regex: Regex = Regex::new(r#"^(0x)?[0-9a-f]{40}$"#).unwrap();

    regex.is_match(&lowercase_address)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // check cli's arguments first
    // might need a better check, but for now it's sufficed.
    // Ignore arguments after required 2 addresses.
    if args.len() < 3 {
        print_program_usage();
        std::process::exit(1);
    }

    // pre-check of address arguments
    if !is_address_simplified(&args[1]) {
        eprintln!("Error: 1st address is malformed. Make sure to prefix with '0x' and has 40 characters in length (exclude `0x`).");
        std::process::exit(1);
    }
    if !is_address_simplified(&args[2]) {
        eprintln!("Error: 2nd address is malformed. Make sure to prefix with '0x' and has 40 characters in length (exclude `0x`).");
        std::process::exit(1);
    }

    // create web3 related context
	let http = match web3::transports::Http::new("https://bsc-dataseed.binance.org/") {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: creating web3's http; err={}", e);
            std::process::exit(1);
        }
    };
    let web3 = web3::Web3::new(http);
    
    // CAVEAT: we cannot do early check whether the input address is indeed
    // a contract address, but until we get response of bytecode of back.
    //
    // If it is contract address -> we will get lengthy of bytecode string
    // If it is EOA address -> empty (0-length string)

    // get bytecode from specified contract address of both arguments
    let contract1_hexbytes_decoded = match hex::decode(&args[1][2..]) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: hex decoding of 1st address; err={}", e);
            std::process::exit(1);
        },
    };
    let contract1_code_fut = web3.eth().code(Address::from_slice(contract1_hexbytes_decoded.as_slice()), None);

    // do the same for 2nd contract address
    let contract2_hexbytes_decoded = match hex::decode(&args[2][2..]) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: hex decoding of 2nd address; err={}", e);
            std::process::exit(1);
        },
    };
    let contract2_code_fut = web3.eth().code(Address::from_slice(contract2_hexbytes_decoded.as_slice()), None);

    // concurrently await both futures, so we don't have to wait until one or another
    // gets response back from RPC call first
    let (c1_code_res, c2_code_res) = futures::join!(contract1_code_fut, contract2_code_fut);

    let c1_code_bytes = match c1_code_res {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: awaiting result for 1st address {}", e);
            std::process::exit(1);
        },
    };
    let c2_code_bytes = match c2_code_res {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error: awaiting result for 2nd address {}", e);
            std::process::exit(1);
        },
    };

    // convert bytes slice into hex string for the contract code
    // NOTE: there is no '0x' prefixed returned for code.
    let c1_code_hex_str = hex::encode(c1_code_bytes.0.as_slice());
    let c2_code_hex_str = hex::encode(c2_code_bytes.0.as_slice());

    // check whether specified contract address 1 & 2 are actually contract address
    // and not EOA
    if c1_code_hex_str.len() == 0 {
        eprintln!("Error: 1st address is **not** a contract address.");
        std::process::exit(1);
    }
    if c2_code_hex_str.len() == 0 {
        eprintln!("Error: 2nd address is **not** a contract address.");
        std::process::exit(1);
    }

    // diff result might be different from `git diff` because seems like
    // the latter use slightly different approach (git uses myers by default along
    // with minimal - seem so) in which
    // `similar` crate doesn't implement `minimal` just yet.
    //
    // CAVEAT: myers algorithm has drawback such that if two contract codes are
    // largely or totally different, it performs very slow.
    let diffs = similar::utils::diff_chars(similar::Algorithm::Myers, &c1_code_hex_str, &c2_code_hex_str);

    for ch in diffs {
        let (val, print_style) = match ch.0 {
            ChangeTag::Equal => (ch.1, Style::new().dim()),     // dim the color on same result
            ChangeTag::Delete => (ch.1, Style::new().red()),
            ChangeTag::Insert => (ch.1, Style::new().green()),
        };

        print!("{}", print_style.apply_to(val));
    }
    println!("");
}
