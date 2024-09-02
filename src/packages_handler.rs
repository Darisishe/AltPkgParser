use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use crate::api_struct::PkgRaw;

/// Data Structure for сonvenient working with packages grouped by architecture
#[derive(Debug)]
pub struct BranchPkgsHandler {
    arch_packages: HashMap<Architecture, HashSet<PkgEntry>>,
}

impl BranchPkgsHandler {
    /// builds Handler using JSON array from "/export/branch_binary_packages/"
    pub(crate) fn from_raw(packages: Vec<PkgRaw>) -> BranchPkgsHandler {
        let mut arch_packages: HashMap<_, HashSet<_>> = HashMap::new();
        for pkg in packages {
            arch_packages.entry(pkg.arch).or_default().insert(PkgEntry {
                name: pkg.name,
                rpm_version: pkg.epoch.to_string() + ":" + &pkg.version + "-" + &pkg.release,
            });
        }

        BranchPkgsHandler { arch_packages }
    }

    /// Iterator over all available architectures for this branch
    pub fn architectures(&self) -> impl Iterator<Item = &Architecture> {
        self.arch_packages.keys()
    }

    /// Returns packages for a given arch
    pub fn packages_iter(&self, arch: &Architecture) -> Option<impl Iterator<Item = &PkgEntry>> {
        self.arch_packages.get(arch).map(|lst| lst.iter())
    }

    /// Checks whether package present for a given arch
    /// # Example
    /// ```
    /// # use altpkgparser::fetch::fetch_branch_packages;
    /// # use altpkgparser::packages_handler::Architecture;
    /// #
    /// # tokio_test::block_on(async {
    /// let handler = fetch_branch_packages("sisyphus").await.unwrap();
    /// assert!(handler.contains(&&Architecture("x86_64".to_owned()), "gcc11"));
    /// assert!(handler.contains(&&Architecture("aarch64".to_owned()), "gcc11"));
    /// assert!(handler.contains(&&Architecture("aarch64".to_owned()), "grep"));
    /// assert!(!handler.contains(&&Architecture("blablabla".to_owned()), "grep"));
    /// assert!(!handler.contains(&&Architecture("x86_64".to_owned()), "blablabla"));
    /// # });
    /// ```
    pub fn contains(&self, arch: &Architecture, pkg_name: &str) -> bool {
        if let Some(pkgs_set) = self.arch_packages.get(arch) {
            pkgs_set.contains(pkg_name)
        } else {
            false
        }
    }

    /// Returns package with a given name and arch
    /// # Example
    /// ```
    /// # use altpkgparser::fetch::fetch_branch_packages;
    /// # use altpkgparser::packages_handler::Architecture;
    /// #
    /// # tokio_test::block_on(async {
    /// let handler = fetch_branch_packages("sisyphus").await.unwrap();
    /// assert!(handler.get_package(&&Architecture("x86_64".to_owned()), "gcc11").is_some());
    /// assert!(handler.get_package(&&Architecture("aarch64".to_owned()), "grep").is_some());
    /// # });
    /// ```
    pub fn get_package(&self, arch: &Architecture, pkg_name: &str) -> Option<&PkgEntry> {
        if let Some(pkgs_set) = self.arch_packages.get(arch) {
            pkgs_set.get(pkg_name)
        } else {
            None
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// Contains just a package name and evr
#[derive(Serialize, Debug, Clone)]
pub struct PkgEntry {
    pub name: String,
    // stores (epoch, version, release) in e:v-r format
    pub rpm_version: String,
}

impl PartialEq for PkgEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PkgEntry {}

impl Hash for PkgEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// implementing this trait, so we can use package name as HashSet key
impl Borrow<str> for PkgEntry {
    fn borrow(&self) -> &str {
        &self.name
    }
}

////////////////////////////////////////////////////////////////////////////////////
/// aliases Arch as String
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct Architecture(pub String);
