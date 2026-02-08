mod cleaner;

use auto_launch::AutoLaunchBuilder;
use std::env;
use std::{thread, time};
use sysinfo::{ProcessesToUpdate, System};

fn setup_auto_launch() {
    let current_exe = env::current_exe().unwrap();
    let auto = AutoLaunchBuilder::new()
        .set_app_name("VRChatCacheCleaner")
        .set_app_path(current_exe.to_str().unwrap())
        .build()
        .unwrap();

    if let Ok(enabled) = auto.is_enabled() {
        if !enabled {
            if let Err(e) = auto.enable() {
                eprintln!("Failed to enable auto-launch: {}", e);
            } else {
                println!("Auto-launch enabled.");
            }
        }
    }
}

fn main() {
    setup_auto_launch();

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
