# contractdiff
Commandline program to output color-diff of contract code between two input
contract addresses. Suitable to use against non-verified contract code for analysis.

# Usage

```
contractdiff --chain <CHAIN> <ADDRESS1> <ADDRESS2>
```

wheres address needs to be prefixed with `0x`.

The tool have internal check, and will report accordingly if address argument
is malformed, or such address is not a contract address, but an EOA address.

# Installation

Install it via `cargo` as follows

`cargo install contractdiff`

# License
MIT, Wasin Thonkaew
