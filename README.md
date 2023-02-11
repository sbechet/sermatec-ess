My small contribution to reverse Sermatec-Ess 光储一体机协议

# CLI Example

```
$ ./sermatec-ess list
--===~ Sermatec ESS CLI ~===--
Asking to 10.10.100.254:8899

protocol version number (pcuVersion): 609
Battery manufacturer number (code list): 30
model code: 2
product_sn (sn): STXXXXXXXXXXXXXXXXXXX
product_sn_ln: 

listing commands:

- [x] sermatec-ess get --el 0a : Battery information display
- [x] sermatec-ess get --el 0b : Control cabinet information display
- [ ] sermatec-ess get --el 0c : Equipment running status
- [x] sermatec-ess get --el 0d : bmsMeter connection status
- [ ] sermatec-ess get --el 1e : BMS alarm information display
- [ ] sermatec-ess get --el 1f : System fault status display
- [x] sermatec-ess get --el 95 : Set parameter query
- [x] sermatec-ess get --el 98 : System Information Query
- [x] sermatec-ess get --el 99 : total power data
- [x] sermatec-ess get --el 9a : Grid power data
- [ ] sermatec-ess get --el 9b : Load power data
- [x] sermatec-ess get --el 9c : Grid battery power data
- [x] sermatec-ess get --el 9d : Set parameter information 2
- [ ] sermatec-ess get --el a1 : Query DRM status
- [ ] sermatec-ess get --el a2 : Forced charge and discharge information
- [x] sermatec-ess get --el b1 : Query routers and servers
- [ ] sermatec-ess get --el bb : Register query
```

```
./sermatec-ess get --el 0a
--===~ Sermatec ESS CLI ~===--
Asking to 10.10.100.254:8899

protocol version number (pcuVersion): 609
Battery manufacturer number (code list): 30
model code: 2
product_sn (sn): STXXXXXXXXXXXXXXXXXXX
product_sn_ln: 

Getting 0a (Battery information display)...
battery voltage: 499
battery current: 0
battery temperature: 112
battery soc: 100
battery soh: 100
Charge and discharge status: 51
maximum allowable charging current: 0
Maximum allowable discharge current: 740
charge cut-off voltage: 532
discharge cut-off voltage: 450
Charge/discharge times: 0
battery pressure: 0
battery warning: 0
battery error: 0
Battery communication connection status: 0
```

```
./sermatec-ess get --el 0b
--===~ Sermatec ESS CLI ~===--
Asking to 10.10.100.254:8899

protocol version number (pcuVersion): 609
Battery manufacturer number (code list): 30
model code: 2
product_sn (sn): STXXXXXXXXXXXXXXXXXXX
product_sn_ln: 

Getting 0b (Control cabinet information display)...
PV1 voltage: 2875
PV1 current: 0
PVI power: 1
PV2 voltage: 2888
PV2 current: 0
PV2 power: 0
Invert A-phase voltage: 2424
Invert phase A current: 4
Grid A phase voltage: 2428
Grid AB line voltage: 0
Grid A phase current: 8
Invert B-phase voltage: 0
Invert B-phase current: 0
Grid B phase voltage: 0
Grid BC line voltage: 0
Grid B-phase current: 0
Invert C-phase voltage: 0
Invert C-phase current: 0
grid phase C voltage: 0
Grid CA line voltage: 0
grid phase C current: 0
grid frequency: 5000
power factor: 40
Grid-side active power: 7
grid-side reactive power: 151
system apparent power: 151
battery current: -4
battery voltage: 508
DC positive bus voltage: 0
DC negative bus voltage: 0
DC bilateral bus voltage: 3799
DC power: -17
internal temperature: 286
10K: DC positive bus backup voltage 5/6K: Secondary bus 1: 2919
10K: DC negative bus backup voltage 5/6K: Secondary bus 2: 2919
device type code: 0
The high digit of the software version number (dspHighVersion): 128
The lower digit of the software version number (dspLowVersion): 0
Parallel address: 0
work efficiency: 0
battery current 1: -4
battery current 2: 0
Module A1 temperature: 173
Module B1 temperature: 181
Module C1 temperature: 0
Load phase A voltage: 2426
Load phase B voltage: 0
Load phase C voltage: 0
load voltage frequency: 5001
load phase A current: 1
load phase B current: 0
load phase C current: 0
load power factor: -576
load active power: 0
load reactive power: -32
load apparent power: 39
Inverter active power (parallel data): 0
Inverter reactive power (parallel data): 0
Invert apparent power (parallel data): 0
Local load active power (parallel data): 0
Local load reactive power (parallel data): 0
Local load apparent power (parallel data): 0
Local load phase A active power (parallel data): 0
Local load B-phase active power (parallel data): 0
Local load phase C active power (parallel data): 0
PV total power (parallel data): 0
Total battery power (parallel data): 0
Total battery current (parallel data): 0
Total battery charging current (parallel data): 0
Total battery discharge current (parallel data): 0
```

# Help WANTED

I'm looking for 5K PCU firmware, specificly for `PCU5KSL_609.bin` please help :)

# TODO

- 0C: bit, bitRange
- 1E: onePosition, preserve
- 1F: onePosition, preserve
- 9B: use repeat field
- A1: bit
- A2: bitRange, preserve
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
