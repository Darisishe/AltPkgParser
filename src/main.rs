use std::collections::HashMap;

use altpkgparser::{
    packages_handler::{Architecture, BranchPkgsHandler, PkgEntry},
    fetch::fetch_branch_packages,
};
use serde_json::{json, to_string_pretty};
use tokio_test::task;

mod data;
use data::{BranchExclusivePkgs, NewerInSisyphusPkgs};

const P10_BRANCH: &str = "p10";
const SISYPHUS_BRANCH: &str = "sisyphus";


/// Gets packages from branch_pkgs, that's not present in other
fn extract_exclusive(branch_pkgs: &BranchPkgsHandler, other: &BranchPkgsHandler) -> Vec<BranchExclusivePkgs> {
    let mut exclusive = Vec::new();
    for arch in branch_pkgs.architectures() {
        if let Some(packages) = branch_pkgs.get_packages(arch) {
            let unique_pkgs = packages.filter(|package| !other.contains(arch, &package.name)).cloned().collect::<Vec<PkgEntry>>();
            exclusive.push(BranchExclusivePkgs {
                arch: arch.clone(),
                packages: unique_pkgs,
            })
        }
    }

    exclusive
}

/// Finds packages whose version-release is greater in sisyphus than in p10
fn get_newer_in_sisyphus(sisyphus_packages: &BranchPkgsHandler, p10_packages: &BranchPkgsHandler) -> Vec<NewerInSisyphusPkgs> {

    vec![]
}

#[tokio::main]
async fn main() {

    // process data in parallel for better performance
    let p10_future = task::spawn(fetch_branch_packages(P10_BRANCH));
    let sisyphus_future = task::spawn(fetch_branch_packages(SISYPHUS_BRANCH));

    let (p10_packages, sisyphus_packages) = tokio::join!(p10_future, sisyphus_future);

    let p10_packages = p10_packages.unwrap();
    let sisyphus_packages = sisyphus_packages.unwrap();

    let p10_exclusive = extract_exclusive(&p10_packages, &sisyphus_packages);
    let sisyphus_exclusive = extract_exclusive(&sisyphus_packages, &p10_packages);
    let newer_in_sisyphus = get_newer_in_sisyphus(&sisyphus_packages, &p10_packages);

    let json = json!({
        "p10_exclusive": p10_exclusive,
        "sisyphus_exclusive": sisyphus_exclusive,
        "newer_in_sisyphus": newer_in_sisyphus
    });

    sisyphus_exclusive.iter();

    println!("{}", to_string_pretty(&json).unwrap());
}
