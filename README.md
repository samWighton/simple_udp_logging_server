# Simple UDP logging server

A simple web server, written in Rust, that logs UDP messages to file

Built as quick solution to get centralised logging in a distributed dev environment

Creates a log file that looks like this:

```
2022-06-26 01:42:18.528808633 UTC|127.0.0.1:48759|{"widget":{"debug":"on","window":{"title":
2022-06-26 01:43:04.672388102 UTC|127.0.0.1:51904|hello there|
2022-06-26 01:44:32.618326564 UTC|204.127.25.52:58495|connection from remote machine|
```

Not written to the standard expected for production use

## To install/run as a linux service

copy built binary to here:

/usr/local/bin/simple_udp_logging_server

relable with this command:
(to keep SELinux happy)
```bash
restorecon -Rv /usr/local/bin
```

Create a service file in this location
/etc/systemd/system/simple_udp_logging_server.service
with this contents

```
[Unit]
Description=logs received UPD packets to file

[Service]
User=root
WorkingDirectory=/tmp/logging/
ExecStart=/usr/local/bin/simple_udp_logging_server
Restart=always

[Install]
WantedBy=multi-user.target
```

once service file is in place, reload the daemon to include it
```bash
systemctl daemon-reload
```

Enable (auto load on restart of machine), start and check status of service
```bash
systemctl enable simple_udp_logging_server.service
systemctl start  simple_udp_logging_server.service
systemctl status simple_udp_logging_server.service
```

## to test

```bash
echo "test message" | nc -u 0.0.0.0 7000
```

