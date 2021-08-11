use crate::operations::garbage_collect_versions;
use crate::config_file::*;
use anyhow::{anyhow, Context, Result};

pub fn run_command_remove(channel: String) -> Result<()> {
    let mut config_data =
        load_config_db().with_context(|| "`remove` command failed to load configuration file.")?;

    if !config_data.installed_channels.contains_key(&channel) {
        return Err(anyhow!("'{}' cannot be removed because it is currently not installed.", channel));
    }

    if let Some(ref default_value) = config_data.default {
        if &channel==default_value {
            return Err(anyhow!("'{}' cannot be removed because it is currently configured as the default channel.", channel));
        }
    }

    config_data.installed_channels.remove(&channel);

    garbage_collect_versions(&mut config_data)?;

    save_config_db(&config_data)
        .with_context(|| format!("Failed to save configuration file from `remove` command after '{}' was installed.", channel))?;

    println!("Julia '{}' successfully removed.", channel);

    Ok(())
}