# Hash Finder - Build & Run

To build the program, use:  
`cargo build --release`

To run the built program directly, use:  
`./target/release/hash_finder -N 3 -F 6`

To run the program, use:  
`cargo run -- -N <N> -F <F>`

### Example:
`cargo run -- -N 2 -F 10`

## Arguments:
- `-N`: The number of trailing zeros in the SHA-256 hash (required).
- `-F`: The number of results to find before stopping (required).

The program will find and display numbers whose SHA-256 hash ends with the specified number of trailing zeros.
