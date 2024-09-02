use altpkgparser::packages_handler::{Architecture, PkgEntry};
use serde::Serialize;

/// Used to describe output format of Branch-unique packages
#[derive(Serialize)]
pub struct BranchExclusivePkgs {
    pub arch: Architecture,
    pub packages: Vec<PkgEntry>,
}

/// Describes output format of packages that are more recent in sisyphus
#[derive(Serialize)]
pub struct NewerInSisyphusPkgs {
    pub arch: Architecture,
    pub packages: Vec<VersionedPkg>,
}

/// Contains info on versions in both branches
#[derive(Serialize)]
pub struct VersionedPkg {
    pub name: String,

    // uses "e:v-r" format
    pub sisyphus_rpm_version: String,
    pub p10_rpm_version: String,
}
