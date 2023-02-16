My small contribution to write a Sermatec-Ess CLI 光储一体机协议

- [x] Tested on SMT-5K-TL-LV hardware with PCU 6.0.9
- [ ] Tested on STM-10K-TL-TH (someone want to test?)

Maybe one day:

- [ ] Configuration commands?
- [ ] Nice GUI using egui?

# CLI Example

```
$ ./sermatec-ess
Usage: sermatec-ess [OPTIONS] [COMMAND]

Commands:
  get     Get a specific things
  list    Get listing of all things
  daemon  Daemon mode use sermatec-ess as a MQTT client
  help    Print this message or the help of the given subcommand(s)

Options:
  -i, --inverter <Inverter IPv4>  Sets Sermatec ESS Ipv4Addr [default: 10.10.100.254]
  -p, --port <Port number>        Sets Sermatec ESS Port number [default: 8899]
  -d, --debug...                  Turn debugging information on
  -h, --help                      Print help
```

```
$ ./sermatec-ess list  
--===~ Sermatec ESS CLI AND MQTT PROXY ~===--
Asking to Sermatec Inverter 10.10.100.254:8899
listing commands:

sermatec-ess get --el 0a : Battery information display
sermatec-ess get --el 0b : Control cabinet information display
sermatec-ess get --el 0c : Equipment running status
sermatec-ess get --el 0d : bmsMeter connection status
sermatec-ess get --el 1e : BMS alarm information display
sermatec-ess get --el 1f : System fault status display
sermatec-ess get --el 64 : Control command settings (*)
sermatec-ess get --el 66 : Operating mode setting (*)
sermatec-ess get --el 67 : Working parameter setting 2 (*)
sermatec-ess get --el 68 : Time Calibration Settings (*)
sermatec-ess get --el 69 : Grid battery type setting (*)
sermatec-ess get --el 6a : Operating mode setting 2 (*)
sermatec-ess get --el 70 : reset (*)
sermatec-ess get --el 71 : Set mandatory charging and discharging information (*)
sermatec-ess get --el 94 : Set WIFI password (*)
sermatec-ess get --el 95 : Set parameter query
sermatec-ess get --el 98 : System Information Query
sermatec-ess get --el 99 : total power data
sermatec-ess get --el 9a : Grid power data
sermatec-ess get --el 9b : Load power data
sermatec-ess get --el 9c : Grid battery power data
sermatec-ess get --el 9d : Set parameter information 2
sermatec-ess get --el 9e : Set router information (*)
sermatec-ess get --el 9f : Set cloud server information (*)
sermatec-ess get --el a1 : Query DRM status
sermatec-ess get --el a2 : Forced charge and discharge information
sermatec-ess get --el a3 : Local WIFI module network configuration (*)
sermatec-ess get --el b0 : Set up routers and servers (*)
sermatec-ess get --el b1 : Query routers and servers
sermatec-ess get --el ba : Register settings (*)
sermatec-ess get --el bb : Register query (*)
(*) DO NOT USE!
```

```
./sermatec-ess get --el 98
--===~ Sermatec ESS CLI AND MQTT PROXY ~===--
Asking to Sermatec Inverter 10.10.100.254:8899
protocol version number: 609
Battery manufacturer number (code list): PYLON Low-voltage Battery 485
model code: 5kW
product_sn: STXXXXXXXXXXXXXXXXXXX
product_sn_ln: 
```

# MQTT Example

```
Daemon mode use sermatec-ess as a MQTT client

Usage: sermatec-ess daemon [OPTIONS] --host <HOST>

Options:
  -m, --host <HOST>  MQTT Server hostname
  -t, --port <PORT>  MQTT Server TCP port [default: 1883]
  -w, --wait <WAIT>  waiting time between two updates (seconds) [default: 300]
  -f, --fork         Detaching from the controlling terminal
  -h, --help         Print help
```

```
./sermatec-ess daemon --host 10.10.100.42 --port 1883
$ ./daemon.sh 
--===~ Sermatec ESS CLI AND MQTT PROXY ~===--
Asking to Sermatec Inverter 10.10.100.254:8899
Sending data to MQTT Daemon 10.10.100.42:1883

MQTT: Sending to sermatec-ess/# Topic...
MQTT: Sending sermatec-ess/grid_battery_power_data/battery_discharge_on_the_day = [30, 2E, 31, 30, 30]
MQTT: Sending sermatec-ess/control_cabinet_information_display/invert_apparent_power_(parallel_data) = [30, 2E, 30, 30, 30]
MQTT: Sending sermatec-ess/set_parameter_information_2/battery_activated = [31]
MQTT: Sending sermatec-ess/set_parameter_information_2/three-phase_unbalanced_output = [30]
MQTT: Sending sermatec-ess/grid_battery_power_data/total_daily_battery_discharge = [31, 35, 2E, 30, 30, 30]
MQTT: Sending sermatec-ess/battery_information_display/battery_soh = [31, 30, 30]
MQTT: Sending sermatec-ess/control_cabinet_information_display/load_reactive_power = [2D, 32, 35, 2E, 30, 30, 30]
MQTT: Sending sermatec-ess/control_cabinet_information_display/local_load_b-phase_active_power_(parallel_data) = [30, 2E, 30, 30, 30]
MQTT: Sending sermatec-ess/control_cabinet_information_display/dc_power = [36, 31, 2E, 30, 30, 30]
MQTT: Sending sermatec-ess/total_power_data/save_money_on_electricity_today = [30, 2E, 30, 30, 30]
MQTT: Notification = Ok(Incoming(ConnAck(ConnAck { session_present: false, code: Success })))
MQTT: Notification = Ok(Outgoing(Subscribe(1)))
MQTT: Notification = Ok(Outgoing(Publish(2)))
MQTT: Notification = Ok(Outgoing(Publish(3)))
MQTT: Notification = Ok(Outgoing(Publish(4)))
MQTT: Notification = Ok(Outgoing(Publish(5)))
MQTT: Notification = Ok(Outgoing(Publish(6)))
MQTT: Notification = Ok(Outgoing(Publish(7)))
MQTT: Notification = Ok(Outgoing(Publish(8)))
MQTT: Notification = Ok(Outgoing(Publish(9)))
MQTT: Notification = Ok(Outgoing(Publish(10)))
MQTT: Notification = Ok(Outgoing(Publish(11)))
MQTT: Notification = Ok(Incoming(SubAck(SubAck { pkid: 1, return_codes: [Success(AtMostOnce)] })))
MQTT: Notification = Ok(Incoming(PubAck(PubAck { pkid: 2 })))
MQTT: Notification = Ok(Incoming(PubAck(PubAck { pkid: 3 })))
MQTT: Sending sermatec-ess/control_cabinet_information_display/load_phase_b_voltage = [30, 2E, 30, 30, 30]
...
```


# Help WANTED

I'm looking for 5K PCU firmware, specificly for `PCU5KSL_609.bin` please help me :)

# TODO

BB is a special query to ask internal registers.

- BB: do not use or reboot! (two parts message)

# Note

## Sermatec in AP mode

(we connect as a client on the Sermatec Interter)

- Sermatec Inverter try to connect to our IP station on TCP/18899 port every second
- We can connect on TCP/8899 port

When TCP Stream is open we can use OSIM protocol

## Sermatec in Station mode

(Sermatec Interter connects itself to the wifi access point)

Sermatec Interter try to connect to IP cloud server on default port 19042 every second.

** THIS IS A SECURITY ISSUE **

- Cloud server can send commands to Sermatec Inverter and reconfigure (or destroy) all directly.
- Cloud server can ask Query routers and servers (B1) to get SSID and PASSWORD AP!

# Denial of Service (DoS) App Access

** THIS IS A SECURITY ISSUE **

Register query (BB) is a two messages parts!
If you send only first message, you block state-macine forever and must reboot.

# Open TCP Ports

- 23/tcp   open  telnet
- 80/tcp   open  http (UART-TCP module web config admin/admin)
- 8000/tcp open  http-alt (?)
- 8899/tcp open  osim (internal) protocol

# Sermatec TCP Port connecting

## Cloud server send...

`8.209.71.159` is Sermatec European Cloud server: you can try.

```
$ netcat -o 19042.txt 8.209.71.159 19042
...
cat 19042_2.bin 
< 00000000 fe 55 64 14 98 00 00 4c ae                      # .Ud....L.
< 00000009 fe 55 64 14 98 00 00 4c ae                      # .Ud....L.
< 00000012 75 9a b0 f9 8a 06 68 85 fc                      # u.....h..
< 0000001b 75 9a b0 f9 8a 06 68 85 fc                      # u.....h..
```

Interesting, because I do not know "75 9a". Maybe for another hardware?
