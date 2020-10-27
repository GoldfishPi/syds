# SYDS
Sort your download sh!t.

Organizes the contents of a directory by extension.

## Install
```bash
$ cargo install syds
```

## Usage

Basic
```bash
$ syds /path/to/dir
```

You can also run syds as a daemon to watch the files in a directory
```bash
$ syds -d /path/to/dir
```

You can also enable syds through a sytemd service
```service
[Unit]
Description=Sorts download directory
Documentation=https://github.com/GoldfishPi/syds

[Service]
ExecStart=%h/.cargo/bin/syds -d %h/Downloads
Restart=always
RestartSec=12

[Install]
WantedBy=default.target
```
