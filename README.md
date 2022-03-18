# bsccontract-diff
Commandline program to output color-diff of contract code between two input
contract addresses. Suitable to use against non-verified contract code for analysis.

# Usage

```
bsccontract-diff 0x00..... 0xab....
```

wheres address needs to be prefixed with `0x`.

The tool have internal check, and will report accordingly if address argument
is malformed, or such address is not a contract address, but an EOA address.

# Installation

Install it via `cargo` as follows

`cargo install bsccontract-diff`

# License
MIT, Wasin Thonkaew
