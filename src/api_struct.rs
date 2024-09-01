use serde::Deserialize;
use crate::data;

/////////////////////////////////////////////////////////////////////////////////////////
// describes the response of "/site/all_pkgset_archs"

#[derive(Deserialize, Debug)]
pub struct PkgsetArchs {
    length: usize,
    pub archs: Vec<ArchData>,
}


#[derive(Deserialize, Debug)]
pub struct ArchData {
    pub arch: data::Architecture,
    count: usize,
}

/////////////////////////////////////////////////////////////////////////////////////////
// describes the response of "/export/branch_binary_packages/"

#[derive(Deserialize, Debug)]
pub struct BranchPkgs {
    pub packages: Vec<PkgDescription>,

    // unused
    requested_args: serde_json::Value,
    length: usize,
}

#[derive(Deserialize, Debug)]
pub struct PkgDescription {
    name: String,
    version: String,

    // unused
    epoch: i32,
    release: String,
    arch: data::Architecture,
    disttag: String,
    buildtime: i32,
    source: String,
}

