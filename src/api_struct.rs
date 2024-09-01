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
