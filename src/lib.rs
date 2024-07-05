use reqwest::blocking::Client;
use std::fs::File;
use std::io;
use std::io::copy;
use std::process::Command;

pub fn run_cmd(command_string: String) -> Result<(), Box<dyn std::error::Error>> {
    // run the command and get the status so output is directed to console
    let status = Command::new("powershell")
        .arg("-c")
        .arg(&command_string)
        .status()
        .expect("Failed to execute command");

    // check the status
    if !status.success() {
        // rust error handling at its finest
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Command failed to run")));
    }

    Ok(())
}

pub fn install_cygwin() -> Result<(), Box<dyn std::error::Error>> {
    // first the cygwin installer must be downloaded

    // create a reqwest client object, then get a response from the download url
    let client = Client::new();
    let mut response = client.get("https://cygwin.com/setup-x86_64.exe").send()?;

    // write the contents of the response to the executable location
    let mut file = File::create("setup-x86_64.exe")?;
    let _ = copy(&mut response, &mut file)?;

    // ensure the file is available before running it
    drop(file);

    println!("Successfully downloaded the Cygwin installer to setup-x86_64.exe.");
    println!("Running the installer. You will not need to interact with it.");

    // use the custom run_cmd(1) function to run the installer
    run_cmd(String::from(".\\setup-x86_64.exe -q -R C:\\cygwin64 -s http://cygwin.mirror.constant.com"))?;

    println!("Downloaded and installed Cygwin successfully!");

    Ok(())
}

pub fn clone_dd() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter source partition name: ");
    let mut src_pname = String::new();
    io::stdin().read_line(&mut src_pname).expect("Failed to read line");
    src_pname = src_pname.trim().parse().unwrap();

    println!("Enter destination partition name: ");
    let mut dest_pname = String::new();
    io::stdin().read_line(&mut dest_pname).expect("Failed to read line");
    dest_pname = dest_pname.trim().parse().unwrap();

    println!("Press any enter to run dd...");
    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).expect("Failed to read line");

    run_cmd(format!("C:\\cygwin64\\bin\\bash --login -c \"dd if=/dev/{} of=/dev/{} bs=4M status=progress\"", src_pname, dest_pname))?;

    Ok(())
}

