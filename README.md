# mqtt2influxdb
MQTT to InfluxDB forwarder in Rust

## Description

This console app could be configured to work as a service.
It subscribes to specified MQTT topics and insert time series events into InfluxDB 
I tested it on x86 linux and ARM7 (raspberry 3 & 4)

## Configuration file

You need to put the configuration file asided the binary file. It should be called mqtt2influxdb.toml.
Sample configuration file included.

## Sample /lib/systemd/system/mqtt2influxdb.service file

```
[Unit]
Description=MQTT to InfluxDB forwarder
After=multi-user.target

[Service]
ExecStart=/home/dietpi/mqtt2influxdb/mqtt2influxdb
Restart=always
RestartSec=10s

[Install]
WantedBy=multi-user.target
```
## Acknowledgments

Many thanks to mqtt-async-client and influx_db_client crates creators for his invaluable work!
[influx_db_client crate](https://crates.io/crates/influx_db_client)
[mqtt-async-client crate](https://crates.io/crates/mqtt-async-client)

