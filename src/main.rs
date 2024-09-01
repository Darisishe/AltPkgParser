use altpkgparser::{
    fetch::fetch_branch_packages,
    packages_handler::BranchPkgsHandler,
};
use rpm::rpm_evr_compare;
use serde_json::{json, to_string_pretty};
use tokio_test::task;

mod data;
use data::{BranchExclusivePkgs, NewerInSisyphusPkgs, VersionedPkg};

const P10_BRANCH: &str = "p10";
const SISYPHUS_BRANCH: &str = "sisyphus";

/// Gets packages from branch_pkgs, that's not present in other
fn extract_exclusive(
    branch_pkgs: &BranchPkgsHandler,
    other: &BranchPkgsHandler,
) -> Vec<BranchExclusivePkgs> {
    let mut exclusive = Vec::new();
    for arch in branch_pkgs.architectures() {
        if let Some(packages) = branch_pkgs.packages_iter(arch) {
            let unique_pkgs = packages.filter(|package| !other.contains(arch, &package.name));

            exclusive.push(BranchExclusivePkgs {
                arch: arch.clone(),
                packages: unique_pkgs.cloned().collect(),
            })
        }
    }

    exclusive
}

/// Finds packages whose version-release is greater in sisyphus than in p10 based on rpm
/// (skips packages that are not in neither Branch)
fn get_newer_in_sisyphus(
    sisyphus_packages: &BranchPkgsHandler,
    p10_packages: &BranchPkgsHandler,
) -> Vec<NewerInSisyphusPkgs> {
    let mut sisyphus_newer = Vec::new();
    for arch in sisyphus_packages.architectures() {
        // list of sisyphus packages for given arch
        let packages = match sisyphus_packages.packages_iter(arch) {
            Some(packages) => packages,
            None => continue,
        };


        let newer_pkgs = packages.filter_map(|sisyphus_pkg| {
            // first check if package present in p10
            match p10_packages.get_package(arch, &sisyphus_pkg.name) {
                Some(p10_pkg) => {
                    // check if sisyphus version is greater
                    match rpm_evr_compare(&sisyphus_pkg.rpm_version, &p10_pkg.rpm_version) {
                        std::cmp::Ordering::Greater => Some(VersionedPkg {
                            name: sisyphus_pkg.name.clone(),
                            sisyphus_rpm_version: sisyphus_pkg.rpm_version.clone(),
                            p10_rpm_version: p10_pkg.rpm_version.clone(),
                        }),
                        _ => None,
                    }
                }
                None => None,
            }
        });

        sisyphus_newer.push(NewerInSisyphusPkgs {
            arch: arch.clone(),
            packages: newer_pkgs.collect(),
        })
    }

    sisyphus_newer
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

    let json_output = json!({
        "p10_exclusive": p10_exclusive,
        "sisyphus_exclusive": sisyphus_exclusive,
        "newer_in_sisyphus": newer_in_sisyphus
    });

    println!("{}", to_string_pretty(&json_output).unwrap());
}
