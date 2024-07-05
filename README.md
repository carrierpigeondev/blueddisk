# Blueddisk
Blueddisk is a Windows tool that utilizes Cygwin and `dd` to clone a drive partition to another.

## Why the name "Blueddisk"?
Well it's a portmanteau of "Blue", "dd", and "disk":
- "Blue" is a reference to the "default Windows color" (ie. the color that is most often associated with Windows).
- "dd" as this uses the `dd` command through Cygwin.
- "disk" since Blueddisk clones one storage disk/drive to another.

## --help (ie. How to Use Blueddisk)
The following is the output of running Blueddisk with the --help or -h flag":
```
Blueddisk is a tool for Windows that utilizes Cygwin and `dd` to clone a drive partition to another.

Usage: blueddisk.exe [OPTIONS]

Options:
  -i, --install-cygwin  Installs Cygwin from https://cygwin.com/setup-x86_64.exe using http://cygwin.mirror.constant.com as the mirror.
  -t, --test-cygwin     Runs the 'whoami' command with Cygwin to ensure Cygwin works.
  -l, --list            Lists all partitions by running 'cat /proc/partitions' with Cygwin.
  -c, --clone           Starts the interactive cloning process, which uses the 'dd' command with Cygwin.
  -h, --help            Print help
  -V, --version         Print version
```
To run with recommended options, use the `-itlc` flag(s).

This program will NOT automatically remove Cygwin. If Cygwin is already installed at `C:\cygwin64`, then it will not reinstall, even with the -i flag. The installer will run and just not do anything. If you have Cygwin installed in a different location, just remove the `C:\cygwin64` directory after Blueddisk runs.