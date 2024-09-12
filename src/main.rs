use std::{collections::HashSet, fmt::format, sync::Arc};

use anyhow::{bail, Context, Result};
use log::*;
use rpm::rpm_evr_compare;
use serde_json::{json, to_writer_pretty};
use structopt::StructOpt;
use tokio_test::task;

use altpkgparser::{
    fetch::{fetch_branch_archs, fetch_branch_packages},
    packages_handler::{Architecture, BranchPkgsHandler},
};

mod data;
use data::{BranchExclusivePkgs, NewerInTargetPkgs, VersionedPkg};

// TODO: Maybe should replace it by getting available branches from API
// Didn't find suitable request: /export/branch_tree/ responce is too large
// and contains some branches, that /export/branch_binary_packages/ doesn't support
const AVAILABLE_BRANCHES: &[&str] = &["p9", "p10", "p11", "sisyphus"];

//////////////////////////////////////////////////////////////////////////////////////
/// Gets packages from branch_pkgs, that's not present in other
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

//////////////////////////////////////////////////////////////////////////////////////
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

    let target_packages =
        target_packages.with_context(|| format!("Failed to request {} packages", target_branch))?;

    let secondary_packages = secondary_packages
        .with_context(|| format!("Failed to request {} packages", secondary_branch))?;

    Ok((target_packages, secondary_packages))
}

/// Requests for Target's and Secondary's archs from API
async fn request_archs(
    target_branch: &str,
    secondary_branch: &str,
) -> Result<(Vec<Architecture>, Vec<Architecture>)> {
    // Just join without spawning new task, because responce is small
    let (target_archs, secondary_archs) = tokio::join!(
        fetch_branch_archs(target_branch),
        fetch_branch_archs(secondary_branch)
    );

    let target_archs = target_archs
        .with_context(|| format!("Failed to request {} architectures", target_branch))?;
    let secondary_archs = secondary_archs
        .with_context(|| format!("Failed to request {} architectures", secondary_branch))?;

    Ok((target_archs, secondary_archs))
}

//////////////////////////////////////////////////////////////////////////////////////
/// Returns a error if some branch is invalid
fn branches_existance_check(branches_names: &[&str]) -> Result<()> {
    for &branch_name in branches_names {
        if !AVAILABLE_BRANCHES.contains(&branch_name) {
            bail!("No such branch: \"{}\"", branch_name);
        }
    }

    Ok(())
}

/// Check if both branches supports given arch (otherwise return error)
async fn arch_presence_check(
    target_branch: &str,
    secondary_branch: &str,
    arch: &Architecture,
) -> Result<()> {
    let (target_archs, secondary_archs) = request_archs(target_branch, secondary_branch).await?;
    if !target_archs.contains(arch) {
        bail!(
            "{} branch doesn't support {} architecture",
            target_branch,
            arch.0
        );
    }
    if !secondary_archs.contains(arch) {
        bail!(
            "{} branch doesn't support {} architecture",
            secondary_branch,
            arch.0
        );
    }

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////////
async fn check_input(
    target_branch: &str,
    secondary_branch: &str,
    arch: Option<&Architecture>,
) -> Result<()> {
    info!(
        "Existence check for \"{}\" and \"{}\" names...",
        target_branch, secondary_branch
    );
    branches_existance_check(&[target_branch, secondary_branch])?;
    if let Some(arch) = arch {
        info!("Checking whether \"{}\" architecture is present in both branches...", arch.0);
        arch_presence_check(target_branch, secondary_branch, arch).await?;
    }

    Ok(())
}

/// All CLI work done here
async fn compare_branches_packages(
    target_branch: &str,
    secondary_branch: &str,
    arch: Option<&Architecture>,
) -> Result<()> {
    // Some checking
    check_input(target_branch, secondary_branch, arch).await?;

    // Fetching packages from API
    info!(
        "Sending requests for {} and {} branches packages to API...",
        target_branch, secondary_branch
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

#[tokio::main]
async fn main() {
    let opts = Opts::from_args();
    setup_logger(opts.verbose);

    if let Err(err) = compare_branches_packages(
        &opts.target_branch,
        &opts.secondary_branch,
        opts.arch.as_ref(),
    )
    .await
    {
        error!("{:#}", err);
        std::process::exit(1);
    }
}
