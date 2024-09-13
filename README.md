# AltPkgParser
## How to run
To run the CLI utility just execute the binary and pass command line args.

Example: `cargo run -- -vvv > output.json` will run the program with max verbosity level, for branches `sisyphus` (as target) and `p10` over all architectures and save output to `output.json`.

*Command line arguments description:*
- `-v` - different levels of logger verbosity (`-v`, `-vv`, `-vvv`)
- `--target` or `-t` - name of Target branch (the one in which newer packages will be searched), `sisyphus` by default. Example: `--target p9`
- `--secondary` or `-s` - name of Secondary branch, `p10` by default. Example: `--secondary p11`
- `--arch` or `-a` - name of architecture for which packages will be processed. By default (no `-a` arg) program will gather information over all architectures. Example `--arch x86_64`

Branch name can be picked from [`p9`, `p10`, `p11`, `sisyphus`].

Example: `cargo run -- -s p9 -t sisyphus -a noarch > output.json` - run silently, with `p9` as Target and `sisyphus` as Secondary, for `noarch` arch.

## Output JSON format
CLI compares lists of `Target` and `Secondary` packages (if `-arch` provided, program only looks for packages for this architecture):
- Finds all packages that are in `Target` but not in `Secondary`
- Finds all packages that are in `Secondary` but not in `Target`
- Finds all packages whose version-release is greater in `Target` than in `Secondary` (based on `rpm`)

And produces output JSON according to the following model:
```json
{
  "newer_in_{target_name}": [
    {
      "arch": "arch_name",
      "packages": [
        {
          "name": "package_name",
          "target_rpm_version": "epoch:version-release",
          "secondary_rpm_version": "epoch:version-release"
        },
        ...
      ]
    },
    ...
  ],

  "{target_name}_exclusive": [
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

  "{secondary_name}_exclusive": [
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

`"newer_in_{target_name}"` - contains packages for each (or only for a given one) architecture (that `Target` branch supports). Each package is described by it's name and `rpm`-versions in both branches.

`"{target_name}_exclusive"` - all packages that are only available in `Target` branch. Each package is described by it's name and `rpm`-version in p10.

`"{secondary_name}_exclusive"` - same as `"{target_name}_exclusive"`

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
