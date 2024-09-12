use crate::api_struct;
use crate::packages_handler::{Architecture, BranchPkgsHandler};
use anyhow::{anyhow, bail, Result};

const PKGS_URL: &str = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

/// Gets packages list by Branch name (and specific arch, if present)
///
/// # Example
///
/// ```
/// # use altpkgparser::fetch::fetch_branch_packages;
/// # use altpkgparser::packages_handler::Architecture;
/// #
/// # tokio_test::block_on(async {
/// let res = fetch_branch_packages("p10", Some(Architecture::from("x86_64")).as_ref()).await.unwrap();
/// let archs = res.architectures().collect::<Vec<_>>();
/// assert!(archs.contains(&&Architecture("x86_64".to_owned())));
/// assert!(!archs.contains(&&Architecture("aarch64".to_owned())));
/// assert!(!archs.contains(&&Architecture("noarch".to_owned())));
/// # });
/// ```
pub async fn fetch_branch_packages(
    branch_name: &str,
    arch: Option<&Architecture>,
) -> Result<BranchPkgsHandler> {
    let mut request_url = PKGS_URL.to_owned() + branch_name;
    if let Some(arch) = arch {
        // Make optimized request for specific architecture
        request_url = request_url + "?arch=" + &arch.0;
    }

    let response = reqwest::get(request_url).await?;
    match response.error_for_status_ref() {
        Ok(_) => {
            // No errors occured during request
            let branch_pkgs: api_struct::BranchPkgsResponse = response.json().await?;

            Ok(BranchPkgsHandler::from_raw(branch_pkgs.packages))
        }
        Err(err) => {
            let text = response.text().await?;

            // Incorrect branch name
            if text.contains("unknown package set name") {
                bail!("No such branch: \"{}\"", branch_name);
            }

            if let Some(arch) = arch {
                // Incorrect arch name
                if text.contains("Invalid architecture name") {
                    bail!(
                        "{} branch doesn't support \"{}\" architecture",
                        branch_name,
                        arch.0
                    );
                }
            }

            // Some other error
            Err(anyhow!(err))
        }
    }
}
