mod colors;
mod desktop;
mod release;
mod system;
mod uptime;

use crate::colors::print_dots;
use crate::desktop::get_desktop_info;
use crate::release::{get_os_pretty_name, get_system_info};
use crate::system::{get_memory_usage, get_root_disk_usage, get_shell, get_username_and_hostname};
use crate::uptime::get_current;
use std::io::{Write, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if Some("--version") == std::env::args().nth(1).as_deref() {
        println!("Microfetch {}", env!("CARGO_PKG_VERSION"));
    } else {
        let utsname = nix::sys::utsname::uname()?;
        let fields = Fields {
            user_info: get_username_and_hostname(&utsname),
            os_name: get_os_pretty_name()?,
            kernel_version: get_system_info(&utsname),
            shell: get_shell(),
            desktop: get_desktop_info(),
            uptime: get_current()?,
            memory_usage: get_memory_usage()?,
            storage: get_root_disk_usage()?,
            colors: print_dots(),
        };
        print_system_info(&fields)?;
    }

    Ok(())
}

struct Fields {
    user_info: String,
    os_name: String,
    kernel_version: String,
    shell: String,
    uptime: String,
    desktop: String,
    memory_usage: String,
    storage: String,
    colors: String,
}

fn print_system_info(fields: &Fields) -> Result<(), Box<dyn std::error::Error>> {
    use crate::colors::COLORS;

    let Fields {
        user_info,
        os_name,
        kernel_version,
        shell,
        uptime,
        desktop,
        memory_usage,
        storage,
        colors,
    } = fields;

    let cyan = COLORS.cyan;
    let blue = COLORS.blue;
    let reset = COLORS.reset;
    let system_info = format!(r"
    {cyan}                    {user_info} ~{reset}
    {cyan}       /\           {cyan}  {blue}System{reset}        {os_name}
    {cyan}      /  \          {cyan}  {blue}Kernel{reset}        {kernel_version}
    {cyan}     /\   \         {cyan}  {blue}Shell{reset}         {shell}
    {cyan}    /    - \        {cyan}  {blue}Uptime{reset}        {uptime}
    {cyan}   /   ''   \       {cyan}  {blue}Desktop{reset}       {desktop}
    {cyan}  /-  |  |  -\      {cyan}  {blue}Memory{reset}        {memory_usage}
    {cyan} /_-''    ''-_\     {cyan}󱥎  {blue}Storage (/){reset}   {storage}
    {cyan}                    {cyan}  {blue}Colors{reset}        {colors}
    ");
    Ok(stdout().write_all(format!("{}\n", system_info).as_bytes())?)
}
