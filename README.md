#WriteBrightness

### Purpose

A secure, simple executable to update the `/sys/class/backlight/intel_backlight/brightness` file.<br>
This file requires root permissions to edit, so `SUDO` or SetUID/SetGID must be used. This executable uses SetUID/SetGID.<br>
Intended for use in shell scripts

### Usage

Build the executable, ```cargo build --release```
Change executable's permissions, ```cd target/release; sudo chown root:root writebrightness; sudo chmod u+s writebrightness; sudo chmod g+s writebrightness;```
Run! ```./writebrightness```

### License

This code is released freely and without warranty.
