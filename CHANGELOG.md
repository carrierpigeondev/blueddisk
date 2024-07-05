# Major Version - 0
## Versions 0.1.x
### 0.1.1
- Fixed major error where user input was not trimmed, so `dd` was fed invalid/nonexistent paths when using `-c`.
- Changed name of clap command to `Blueddisk` from `blueddisk` to better fit user facing name, rather than technical name.
- clap command version now uses Cargo environment variable to match with version in Cargo.toml
- Added Cargo.lock to `.gitignore`.
- Added `.idea` directory to `.gitignore`. Still, Rust Rover ftw!
- Added `CHANGELOG.md`.

### 0.1.0
Initial release:
- Can download Cygwin installer and automatically run installer with prechosen settings (root as `C:\cygwin64` and mirror site as `http://cygwin.mirror.constant.com`) with `-i`.
- `whoami` as a test command with `-t`.
- Lists all partitions using `cat /proc/partitions` with `-l`
- Interactive clone functionality with `-c` using `dd` with args from user in the interactive cli. Does not work do to error getting user input.