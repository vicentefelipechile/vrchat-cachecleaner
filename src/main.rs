#![windows_subsystem = "windows"]

mod cleaner;

use auto_launch::AutoLaunchBuilder;
use notify_rust::Notification;
use single_instance::SingleInstance;
use std::env;
use std::{thread, time};
use sysinfo::{ProcessesToUpdate, System};
use winreg::RegKey;
use winreg::enums::*;

fn setup_auto_launch() {
    let current_exe = env::current_exe().unwrap();
    let current_exe_str = current_exe.to_str().unwrap();

    let auto = AutoLaunchBuilder::new()
        .set_app_name("VRChatCacheCleaner")
        .set_app_path(current_exe_str)
        .build()
        .unwrap();

    // Check existing registry entry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = std::path::Path::new("Software")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Run");

    let should_enable = if let Ok(key) = hkcu.open_subkey(&path) {
        if let Ok(value) = key.get_value::<String, _>("VRChatCacheCleaner") {
            value != current_exe_str
        } else {
            true
        }
    } else {
        true
    };

    if should_enable {
        if let Err(e) = auto.enable() {
            eprintln!("Failed to update auto-launch: {}", e);
        } else {
            println!("Auto-launch updated to current path.");
        }
    } else {
        println!("Auto-launch already set correctly.");
    }
}

fn main() {
    let instance = SingleInstance::new("vrchat-cachecleaner-lock").unwrap();
    if !instance.is_single() {
        if let Err(e) = Notification::new()
            .summary("VRChat Cache Cleaner")
            .body("App is already running.")
            .show()
        {
            eprintln!("Failed to show notification: {}", e);
        }
        return;
    }

    setup_auto_launch();

    if let Err(e) = Notification::new()
        .summary("VRChat Cache Cleaner")
        .body("Cleaner is running in the background.")
        .show()
    {
        eprintln!("Failed to show notification: {}", e);
    }

    let mut system = System::new_all();
    let mut was_running = false;

    println!("VRChat Cache Cleaner started. Monitoring for VRChat.exe...");

    loop {
        // Refresh processes
        system.refresh_processes(ProcessesToUpdate::All, true);

        // Check if VRChat is running
        // processes_by_exact_name returns an iterator
        let is_running = system
            .processes_by_exact_name("VRChat.exe".as_ref())
            .next()
            .is_some();

        if is_running {
            if !was_running {
                println!("VRChat started.");
            }
            was_running = true;
        } else {
            if was_running {
                println!("VRChat closed. Cleaning cache...");
                // Wait a bit to ensure the process is fully released
                thread::sleep(time::Duration::from_secs(5));
                if let Err(e) = cleaner::clean_cache() {
                    eprintln!("Error cleaning cache: {}", e);
                }
                was_running = false;
            }
        }

        thread::sleep(time::Duration::from_secs(10));
    }
}
