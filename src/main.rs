mod lib;

use clap::{Arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");  // version from Cargo.toml

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Blueddisk")
        .version(VERSION)
        .about("Blueddisk is a tool for Windows that utilizes Cygwin and `dd` to clone a drive partition to another.")
        .arg(
            Arg::new("install-cygwin")
                .long("install-cygwin")
                .short('i')
                .help("Installs Cygwin from https://cygwin.com/setup-x86_64.exe using http://cygwin.mirror.constant.com as the mirror.")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("test-cygwin")
                .long("test-cygwin")
                .short('t')
                .help("Runs the 'whoami' command with Cygwin to ensure Cygwin works.")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .help("Lists all partitions by running 'cat /proc/partitions' with Cygwin.")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("clone")
                .long("clone")
                .short('c')
                .help("Starts the interactive cloning process, which uses the 'dd' command with Cygwin.")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    if *matches.get_one::<bool>("install-cygwin").unwrap_or(&false) {
        println!("Installing Cygwin.");
        lib::install_cygwin()?;
    }

    if *matches.get_one::<bool>("test-cygwin").unwrap_or(&false) {
        println!("Testing Cygwin with 'whoami'.");
        lib::run_cmd(String::from("C:\\cygwin64\\bin\\bash --login -c \"whoami\""))?;
    }

    if *matches.get_one::<bool>("list").unwrap_or(&false) {
        println!("Listing all partitions.");
        lib::run_cmd(String::from("C:\\cygwin64\\bin\\bash --login -c \"cat /proc/partitions\""))?;
        println!("Make sure the destination partition is the equal to or larger than the source.")
    }

    if *matches.get_one::<bool>("clone").unwrap_or(&false) {
        lib::clone_dd()?;
    }

    Ok(())
}
