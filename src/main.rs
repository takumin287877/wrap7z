#![windows_subsystem = "windows"]

use std::env;
use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    Command::new(r"C:\Program Files\7-Zip\7z.exe")
        .args(["x", &args[1], "-o*", "-aoa"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .ok();
}
