use crate::colors::COLORS;
use nix::sys::{statvfs::statvfs, utsname::UtsName};
use std::{
    env,
    fs::File,
    io::{self, Read},
};

pub fn get_username_and_hostname(utsname: &UtsName) -> String {
    let username = env::var("USER").unwrap_or_else(|_| "unknown_user".to_string());
    let hostname = utsname
        .nodename()
        .to_str()
        .unwrap_or("unknown_host")
        .to_string();
    format!(
        "{yellow}{username}{red}@{green}{hostname}{reset}",
        yellow = COLORS.yellow,
        red = COLORS.red,
        green = COLORS.green,
        reset = COLORS.reset,
    )
}

pub fn get_shell() -> String {
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "unknown_shell".to_string());
    let shell_name = shell_path.rsplit('/').next().unwrap_or("unknown_shell");
    shell_name.to_string()
}

pub fn get_root_disk_usage() -> Result<String, io::Error> {
    let vfs = statvfs("/")?;
    let block_size = vfs.block_size() as u64;
    let total_blocks = vfs.blocks();
    let available_blocks = vfs.blocks_available();

    let total_size = block_size * total_blocks;
    let used_size = total_size - (block_size * available_blocks);

    let total_size = total_size as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_size = used_size as f64 / (1024.0 * 1024.0 * 1024.0);
    let usage = (used_size / total_size) * 100.0;

    Ok(format!(
        "{used_size:.2} GiB / {total_size:.2} GiB ({cyan}{usage:.0}%{reset})",
        cyan = COLORS.cyan,
        reset = COLORS.reset,
    ))
}

pub fn get_memory_usage() -> Result<String, io::Error> {
    fn parse_memory_info() -> Result<(f64, f64), io::Error> {
        let mut total_memory_kb = 0.0;
        let mut available_memory_kb = 0.0;
        let mut meminfo = String::with_capacity(2048);

        File::open("/proc/meminfo")?.read_to_string(&mut meminfo)?;

        for line in meminfo.lines() {
            let mut split = line.split_whitespace();
            match split.next().unwrap_or_default() {
                "MemTotal:" => total_memory_kb = split.next().unwrap_or("0").parse().unwrap_or(0.0),
                "MemAvailable:" => {
                    available_memory_kb = split.next().unwrap_or("0").parse().unwrap_or(0.0);
                    // MemTotal comes before MemAvailable, stop parsing
                    break;
                }
                _ => (),
            }
        }

        let total_memory_gb = total_memory_kb / 1024.0 / 1024.0;
        let available_memory_gb = available_memory_kb / 1024.0 / 1024.0;
        let used_memory_gb = total_memory_gb - available_memory_gb;

        Ok((used_memory_gb, total_memory_gb))
    }

    let (used_memory, total_memory) = parse_memory_info()?;
    let percentage_used = (used_memory / total_memory * 100.0).round() as u64;

    Ok(format!(
        "{used_memory:.2} GiB / {total_memory:.2} GiB ({cyan}{percentage_used}%{reset})",
        cyan = COLORS.cyan,
        reset = COLORS.reset,
    ))
}
