My small contribution to reverse Sermatec-Ess 光储一体机协议

** ALL "STANDARD" QUERY WORKS **

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

sermatec-ess get --el 0a : Battery information display
sermatec-ess get --el 0b : Control cabinet information display
sermatec-ess get --el 0c : Equipment running status
sermatec-ess get --el 0d : bmsMeter connection status
sermatec-ess get --el 1e : BMS alarm information display
sermatec-ess get --el 1f : System fault status display
sermatec-ess get --el 95 : Set parameter query
sermatec-ess get --el 98 : System Information Query
sermatec-ess get --el 99 : total power data
sermatec-ess get --el 9a : Grid power data
sermatec-ess get --el 9b : Load power data
sermatec-ess get --el 9c : Grid battery power data
sermatec-ess get --el 9d : Set parameter information 2
sermatec-ess get --el a1 : Query DRM status
sermatec-ess get --el a2 : Forced charge and discharge information
sermatec-ess get --el b1 : Query routers and servers
sermatec-ess get --el bb : Register query
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
battery voltage: 49.6 V
battery current: 2.6 A
battery temperature: 10.4 C
battery soc: 90
battery soh: 100
Charge and discharge status: 34
maximum allowable charging current: 51.800000000000004 A
Maximum allowable discharge current: 74 A
charge cut-off voltage: 53.2 V
discharge cut-off voltage: 45 V
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
PV1 voltage: 9.3 V
PV1 current: 0 A
PVI power: 0 W
PV2 voltage: 9.3 V
PV2 current: 0 A
PV2 power: 0 W
Invert A-phase voltage: 240.60000000000002 V
Invert phase A current: 0.5 A
Grid A phase voltage: 241.20000000000002 V
Grid AB line voltage: 0 V
Grid A phase current: 0.8 A
Invert B-phase voltage: 0 V
Invert B-phase current: 0 A
Grid B phase voltage: 0 V
Grid BC line voltage: 0 V
Grid B-phase current: 0 A
Invert C-phase voltage: 0 V
Invert C-phase current: 0 A
grid phase C voltage: 0 V
Grid CA line voltage: 0 V
grid phase C current: 0 A
grid frequency: 50.03 HZ
power factor: -0.03
Grid-side active power: -6 W
grid-side reactive power: 175 W
system apparent power: 175 Var
battery current: 2.2 A
battery voltage: 50.400000000000006 V
DC positive bus voltage: 0 V
DC negative bus voltage: 0 V
DC bilateral bus voltage: 379.90000000000003 V
DC power: 101 W
internal temperature: 27.900000000000002 ℃
10K: DC positive bus backup voltage 5/6K: Secondary bus 1: 294.1 V
10K: DC negative bus backup voltage 5/6K: Secondary bus 2: 295 V
device type code: 0
The high digit of the software version number (dspHighVersion): 128
The lower digit of the software version number (dspLowVersion): 0
Parallel address: 0
work efficiency: 0
battery current 1: 1 A
battery current 2: 1.1 A
Module A1 temperature: 16.1 ℃
Module B1 temperature: 15.8 ℃
Module C1 temperature: 0 ℃
Load phase A voltage: 240.8 V
Load phase B voltage: 0 V
Load phase C voltage: 0 V
load voltage frequency: 50.03 HZ
load phase A current: 0.1 A
load phase B current: 0 A
load phase C current: 0 A
load power factor: -0.6990000000000001
load active power: 0 VA
load reactive power: -22 Var
load apparent power: 32 W
Inverter active power (parallel data): 0 KW
Inverter reactive power (parallel data): 0 KVar
Invert apparent power (parallel data): 0 KW
Local load active power (parallel data): 0 KW
Local load reactive power (parallel data): 0 KVar
Local load apparent power (parallel data): 0 KW
Local load phase A active power (parallel data): 0 KW
Local load B-phase active power (parallel data): 0 KW
Local load phase C active power (parallel data): 0 KW
PV total power (parallel data): 0 KW
Total battery power (parallel data): 0 KW
Total battery current (parallel data): 0 A
Total battery charging current (parallel data): 0 A
Total battery discharge current (parallel data): 0 A
```

# Help WANTED

I'm looking for 5K PCU firmware, specificly for `PCU5KSL_609.bin` please help :)

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
