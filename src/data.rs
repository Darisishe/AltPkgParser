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

    pub sisyphus_version: String,
    pub sisyphus_release: String,

    pub p10_version: String,
    pub p10_release: String,
}