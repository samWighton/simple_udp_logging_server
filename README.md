# Simple UDP logging server

## to install as a service

copy built binary to here:

/usr/local/bin/simple_udp_logging_server

relable with this command:
(to keep SELinux happy)
```bash
restorecon -Rv /usr/local/bin
```

Create service file in this location
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

