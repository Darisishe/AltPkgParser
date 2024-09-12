use anyhow::{Context, Result};
use log::*;
use rpm::rpm_evr_compare;
use serde_json::{json, to_writer_pretty};
use structopt::StructOpt;
use tokio_test::task;

use altpkgparser::{
    fetch::fetch_branch_packages,
    packages_handler::{Architecture, BranchPkgsHandler},
};

mod data;
use data::{BranchExclusivePkgs, NewerInTargetPkgs, VersionedPkg};

//////////////////////////////////////////////////////////////////////////////////////
/// Gets packages from branch_pkgs, that are not present in other
fn extract_exclusive(
    branch_pkgs: &BranchPkgsHandler,
    other: &BranchPkgsHandler,
) -> Vec<BranchExclusivePkgs> {
    let mut exclusive = Vec::new();
    for arch in branch_pkgs.architectures() {
        if let Some(packages) = branch_pkgs.packages_iter(arch) {
            // filter out packages that present in other
            let unique_pkgs = packages.filter(|package| !other.contains(arch, &package.name));

            exclusive.push(BranchExclusivePkgs {
                arch: arch.clone(),
                packages: unique_pkgs.cloned().collect(),
            })
        }
    }

    exclusive
}

/// Finds packages whose version-release is greater in Target branch than in Secondary based on rpm
/// (skips packages that are not in any of Branch)
fn get_newer_in_target(
    target_packages: &BranchPkgsHandler,
    secondary_packages: &BranchPkgsHandler,
) -> Vec<NewerInTargetPkgs> {
    let mut target_newer = Vec::new();
    for arch in target_packages.architectures() {
        // list of target packages for given arch
        let packages = match target_packages.packages_iter(arch) {
            Some(packages) => packages,
            None => continue,
        };

        let newer_pkgs = packages.filter_map(|target_pkg| {
            // first check if package present in Secondary
            match secondary_packages.get_package(arch, &target_pkg.name) {
                Some(sec_pkg) => {
                    // check if Target version is greater
                    match rpm_evr_compare(&target_pkg.rpm_version, &sec_pkg.rpm_version) {
                        std::cmp::Ordering::Greater => Some(VersionedPkg {
                            name: target_pkg.name.clone(),
                            target_rpm_version: target_pkg.rpm_version.clone(),
                            secondary_rpm_version: sec_pkg.rpm_version.clone(),
                        }),
                        _ => None,
                    }
                }
                None => None,
            }
        });

        target_newer.push(NewerInTargetPkgs {
            arch: arch.clone(),
            packages: newer_pkgs.collect(),
        })
    }

    target_newer
}

//////////////////////////////////////////////////////////////////////////////////////
/// Provides context for request result in case of error
fn provide_context(
    request_result: Result<BranchPkgsHandler>,
    branch_name: &str,
) -> Result<BranchPkgsHandler> {
    request_result.with_context(|| format!("Failed to request {} packages", branch_name))
}

/// Requests for Target's and Secondary's packages (in parallel) from API and build Handlers
async fn request_packages(
    target_branch: &str,
    secondary_branch: &str,
    arch: Option<&Architecture>,
) -> Result<(BranchPkgsHandler, BranchPkgsHandler)> {
    // process data in parallel for better performance
    let target_future = task::spawn(fetch_branch_packages(target_branch, arch));
    let secondary_future = task::spawn(fetch_branch_packages(secondary_branch, arch));

    // wait for fetched data
    let (target_packages, secondary_packages) = tokio::join!(target_future, secondary_future);

    let target_packages = provide_context(target_packages, target_branch)?;
    let secondary_packages = provide_context(secondary_packages, secondary_branch)?;

    Ok((target_packages, secondary_packages))
}

//////////////////////////////////////////////////////////////////////////////////////
/// All CLI work done here
async fn compare_branches_packages(
    target_branch: &str,
    secondary_branch: &str,
    arch: Option<&Architecture>,
) -> Result<()> {
    // Fetching packages from API
    info!(
        "Sending requests for {} and {} branches packages for {} to API...",
        target_branch,
        secondary_branch,
        if let Some(arch) = arch {
            &arch.0
        } else {
            "all architectures"
        }
    );

    let (target_packages, secondary_packages) =
        request_packages(target_branch, secondary_branch, arch).await?;
    info!("Branches packages fetched successfully!");
    trace!(
        "{} architectures: {:?}",
        target_branch,
        target_packages.architectures().collect::<Vec<_>>()
    );
    trace!(
        "{} architectures: {:?}",
        secondary_branch,
        secondary_packages.architectures().collect::<Vec<_>>()
    );

    // Comparing packages lists
    info!("Proccessing packages...");
    let target_exclusive = extract_exclusive(&target_packages, &secondary_packages);
    let secondary_exclusive = extract_exclusive(&secondary_packages, &target_packages);
    let newer_in_target = get_newer_in_target(&target_packages, &secondary_packages);

    // Print output
    info!("Producing output...");
    let json_output = json!({
        format!("{}_exclusive", target_branch): target_exclusive,
        format!("{}_exclusive", secondary_branch): secondary_exclusive,
        format!("newer_in_{}", target_branch): newer_in_target
    });
    to_writer_pretty(std::io::stdout(), &json_output).context("Failed to produce output JSON")?;

    info!("All done!");
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////
/// Command line arguments
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opts {
    /// Target branch (program will extract packages that are newer in Target than in Secondary)
    #[structopt(short = "t", long = "target", default_value = "sisyphus")]
    target_branch: String,

    /// Secondary branch
    #[structopt(short = "s", long = "secondary", default_value = "p10")]
    secondary_branch: String,

    /// Architecture (optional argument)
    #[structopt(short = "a", long = "arch", parse(from_str))]
    arch: Option<Architecture>,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

/// Configures simple stderr logger
fn setup_logger(verbose: usize) {
    stderrlog::new()
        .verbosity(1 + verbose)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .expect("failed to initialize logging");
}

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    setup_logger(opts.verbose);

    let comparison_future = compare_branches_packages(
        &opts.target_branch,
        &opts.secondary_branch,
        opts.arch.as_ref(),
    );

    if let Err(err) = comparison_future.await {
        error!("{:#}", err);
        std::process::exit(1);
    }
}
