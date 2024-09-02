# AltPkgParser
## How to run
To run the CLI utility just execute the binary and pass command line args (`-v`, `-vv`, `-vvv`, maybe none) - different levels of logger verbosity.

Example: `cargo run -- -vvv > output.json` will run the program with max verbosity level and save output to `output.json`. 

## Output JSON format
CLI compares lists of p10 and sisyphus packages:
- Finds all packages that are in p10 but not in sisyphus
- Finds all packages that are in sisyphus but not in p10
- Finds all packages whose version-release is greater in sisyphus than in p10 (based on `rpm`)

And produces output JSON according to the following model:
```json
{
  "newer_in_sisyphus": [
    {
      "arch": "arch_name",
      "packages": [
        {
          "name": "package_name",
          "p10_rpm_version": "epoch:version-release",
          "sisyphus_rpm_version": "epoch:version-release"
        },
        ...
      ]
    },
    ...
  ],

  "p10_exclusive": [
    {
      "arch": "arch_name",
      "packages": [
        {
          "name": "package_name",
          "rpm_version": "epoch:version-release"
        },
        ...
      ]
    },
    ...
  ],

  "sisyphus_exclusive": [
    {
      "arch": "arch_name",
      "packages": [
        {
          "name": "package_name",
          "rpm_version": "epoch:version-release"
        },
        ...
      ]
    },
    ...
  ]
}
```

`"newer_in_sisyphus"` - contains packages for each architecture (that sisyphus supports). Each package is described by it's name and `rpm`-versions in both branches.

`"p10_exclusive"` - all packages that are only available in p10 (for each architecture that p10 supports). Each package is described by it's name and `rpm`-version in p10.

`"sisyphus_exclusive"` - same as `"p10_exclusive"`

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