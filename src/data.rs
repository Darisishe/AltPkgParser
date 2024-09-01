use altpkgparser::packages_handler::{Architecture, PkgEntry};
use serde::Serialize;

#[derive(Serialize)]
pub struct BranchExclusivePkgs {
    pub arch: Architecture,
    pub packages: Vec<PkgEntry>
}

#[derive(Serialize)]
pub struct NewerInSisyphusPkgs {
    pub arch: Architecture,
    pub packages: Vec<VersionedPkg>
}

#[derive(Serialize)]
pub struct VersionedPkg {
    pub name: String,

    // uses "e:v-r" format
    pub sisyphus_rpm_version: String,
    pub p10_rpm_version: String,
}