# AltPkgParser
## How to run
To run the CLI utility just execute the binary and pass command line args (`-v`, `-vv`, `-vvv`, maybe none) - different levels of logger verbosity.

Example: `cargo run -- -vvv > output.json` will run the program with max verbosity level and save output to `output.json`. 

## Output JSON format

## Project structure
Crate is splitted into two parts: Library and Binary.

Library:
* `api_struct.rs` - Describes API Responses JSON structure 
* `packages_handler.rs` - Provides convinience Data Structure for working with packages
* `fetch.rs` - Implements functionality for fetching packages from API
* `lib.rs` - Gathers all modules together

Binary (CLI):
* `data.rs` - Contains structures for output JSON format for packages comparison results
* `main.rs` - CLI Implementation

## Details
Project uses `reqwest`-crate and `tokio`-runtime to access API. Errors are handled using `anyhow` and logging.