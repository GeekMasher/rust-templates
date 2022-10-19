
// Import core GHActions macros
use ghactions::{group, groupend};
// Logging functions
use log::{info, debug, warn};
// Easy Error enum magic
use std::error::Error;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut action = ghactions::init();

    if ! action.in_action() {
        warn!("Failed to load action.yml file");
    }

    debug!("GitHub Action Name :: {}", &action.name.clone().unwrap_or_else(|| "N/A".to_string()));

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository.display());

    let client = action.client
        .expect("Failed loading client...");

    // https://github.com/softprops/hubcaps/blob/master/examples/releases.rs
    let repo = client.repo(action.repository.owner, action.repository.name);

    let latest = repo.releases().latest().await?;
    info!("{:#?}", latest);

    for r in repo.releases().list().await? {
        info!("  -> {}", r.name);
    }

    groupend!();

    Ok(())
}
