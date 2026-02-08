use anyhow::{Context, Result};
use directories::BaseDirs;
use std::fs;

pub fn clean_cache() -> Result<()> {
    let base_dirs = BaseDirs::new().context("Could not find base directories")?;
    let data_local_dir = base_dirs.data_local_dir();
    // VRChat uses LocalLow, not Local. We navigate up one level from Local (AppData/Local) to AppData and then into LocalLow.
    let data_local_low_dir = data_local_dir
        .parent()
        .context("Could not find parent of data_local_dir")?
        .join("LocalLow");

    // Paths to clean
    let cache_paths = vec![
        data_local_low_dir.join("VRChat\\VRChat\\Cache-WindowsPlayer"),
        data_local_low_dir.join("VRChat\\VRChat\\HTTPCache-WindowsPlayer"),
        data_local_low_dir.join("VRChat\\VRChat\\TextureCache-WindowsPlayer"),
    ];

    for path in cache_paths {
        if path.exists() {
            println!("Cleaning: {:?}", path);
            // In a real scenario, we might want to be robust against open files, etc.
            if let Err(e) = fs::remove_dir_all(&path) {
                eprintln!("Failed to remove {:?}: {}", path, e);
            } else {
                println!("Successfully removed {:?}", path);
            }
        } else {
            println!("Path not found, skipping: {:?}", path);
        }
    }

    Ok(())
}
