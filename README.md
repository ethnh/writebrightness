# WriteBrightness

### Purpose

A secure, simple executable to update the `/sys/class/backlight/intel_backlight/brightness` file.<br>
This file requires root permissions to edit, so `SUDO` or SetUID/SetGID must be used.<br>This executable uses SetUID/SetGID.<br>
Intended for use in shell scripts

### Usage

Build the executable,<br>```cargo build --release```<br><br>
Change executable's permissions,<br>```cd target/release; sudo chown root:root writebrightness; sudo chmod u+s writebrightness; sudo chmod g+s writebrightness;```<br><br>
Run!<br>```./writebrightness```

### License

This code is released freely and without warranty.
