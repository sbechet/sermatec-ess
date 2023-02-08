My small contribution to reverse Sermatec-Ess 光储一体机协议

# Proxmox 6.4 Wifi Network configuration

## IWD Configuration

```
systemctl --now disable wpa_supplicant
killall wpa_supplicant
apt remove wpasupplicant
apt install iwd
systemctl --now enable iwd
iwctl
[iwd]# device list
[iwd]# device wlp1s0 Powered on
[iwd]# station list
[iwd]# station wlp1s0 get-networks
[iwd]# station wlp1s0 connect $MYNETWORK $MYPASSWORD
[iwd]# exit
```

## dhcp client configuration

Only for iwd < 0.19, we must use dhcpcd:

```
apt install dhcpcd5
echo allowinterfaces wlp1s0 >> /etc/dhcpcd.conf
systemctl restart dhcpcd
```

## /etc/network/interfaces

add to interfaces configuration:

```
auto wlp1s0
iface wlp1s0 inet dhcp
        post-up   iptables -t nat -A POSTROUTING -s '192.168.0.0/24' -o wlp1s0 -j MASQUERADE
        post-down iptables -t nat -D POSTROUTING -s '192.168.0.0/24' -o wlp1s0 -j MASQUERADE
```


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

Cloud server can send commands to Sermatec Inverter and reconfigure (or destroy) all directly.


# Nmap

```
nmap 10.10.100.254
Starting Nmap 7.70 ( https://nmap.org ) at 2023-01-28 18:19 CET
Nmap scan report for 10.10.100.254
Host is up (0.021s latency).
Not shown: 996 closed ports
PORT     STATE SERVICE
23/tcp   open  telnet
80/tcp   open  http
8000/tcp open  http-alt
8899/tcp open  ospf-lite
MAC Address: 9C:A5:25:D5:2E:96 (Shandong USR IOT Technology Limited)

Nmap done: 1 IP address (1 host up) scanned in 17.16 seconds
```

# Example

`$ ./sermatec-ess list`

```
# Inverter

10.10.100.254:8899

# JSON Command

Command {
    cmd: "98",
    comment: "System Information Query",
    op: 1,
    fields: [
        Field {
            order: 0,
            byte_len: 2,
            byte_order: 0,
            tag: "pcuVersion",
            type_type: "int",
            name: "protocol version number",
            converter: "",
            validate: "",
            unit_type: "",
            unit_value: "",
            precision: 0,
            group: 0,
            group_tag: "",
            repeat: 0,
            repeat_ref: 0,
            repeat_group: 0,
            from_bit: 0,
            end_bit: 0,
            bit_position: 0,
            same: false,
            default_value: "",
            return_value: "",
        },
        Field {
            order: 1,
            byte_len: 2,
            byte_order: 0,
            tag: "",
            type_type: "int",
            name: "Battery manufacturer number (code list)",
            converter: "",
            validate: "",
            unit_type: "",
            unit_value: "",
            precision: 0,
            group: 0,
            group_tag: "",
            repeat: 0,
            repeat_ref: 0,
            repeat_group: 0,
            from_bit: 0,
            end_bit: 0,
            bit_position: 0,
            same: false,
            default_value: "",
            return_value: "",
        },
        Field {
            order: 2,
            byte_len: 2,
            byte_order: 0,
            tag: "",
            type_type: "int",
            name: "model code",
            converter: "",
            validate: "",
            unit_type: "",
            unit_value: "",
            precision: 0,
            group: 0,
            group_tag: "",
            repeat: 0,
            repeat_ref: 0,
            repeat_group: 0,
            from_bit: 0,
            end_bit: 0,
            bit_position: 0,
            same: false,
            default_value: "",
            return_value: "",
        },
        Field {
            order: 3,
            byte_len: 26,
            byte_order: 0,
            tag: "sn",
            type_type: "string",
            name: "product_sn",
            converter: "",
            validate: "",
            unit_type: "",
            unit_value: "",
            precision: 0,
            group: 0,
            group_tag: "",
            repeat: 0,
            repeat_ref: 0,
            repeat_group: 0,
            from_bit: 0,
            end_bit: 0,
            bit_position: 0,
            same: false,
            default_value: "",
            return_value: "",
        },
        Field {
            order: 4,
            byte_len: 18,
            byte_order: 0,
            tag: "",
            type_type: "string",
            name: "product_sn_ln",
            converter: "",
            validate: "",
            unit_type: "",
            unit_value: "",
            precision: 0,
            group: 0,
            group_tag: "",
            repeat: 0,
            repeat_ref: 0,
            repeat_group: 0,
            from_bit: 0,
            end_bit: 0,
            bit_position: 0,
            same: false,
            default_value: "",
            return_value: "",
        },
    ],
}

# Question

[fe, 55, 64, 14, 98, 0, 0, 4c, ae]

# Answer:

[fe, 55, 14, 64, 98, 0, 32, 2, 61, 0, 1e, 0, 2, 53, 54, 33, 31, ... , 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, ae]

# System Information Query

Ok(
    [
        (
            "pcuVersion",
            "protocol version number",
            Int(
                609,
            ),
        ),
        (
            "",
            "Battery manufacturer number (code list)",
            Int(
                30,
            ),
        ),
        (
            "",
            "model code",
            Int(
                2,
            ),
        ),
        (
            "sn",
            "product_sn",
            String(
                "ST31XXXXXXXXX",
            ),
        ),
        (
            "",
            "product_sn_ln",
            String(
                "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
        ),
    ],
)

listing all...
TODO, check with current_version
# Version 0
## Command 98() System Information Query
TODO, check with current_version
# Version 252
## Command 0A() Battery information display
## Command 0B() Control cabinet information display
## Command 0C() Equipment running status
## Command 1E() BMS alarm information display
## Command 1F() System fault status display
## Command 99() total power data
## Command 9A() Grid power data
## Command 9B() Load power data
## Command 95() Set parameter query
TODO, check with current_version
# Version 258
## Command 0D() bmsMeter connection status
## Command 1E() BMS alarm information display
## Command 9C() Grid battery power data
## Command A1() Query DRM status
## Command 95() Set parameter query
TODO, check with current_version
# Version 259
## Command 95() Set parameter query
## Command 9D() Set parameter information 2
TODO, check with current_version
# Version 500
## Command A2() Forced charge and discharge information
## Command 9D() Set parameter information 2
## Command 0B() Control cabinet information display
## Command B1() Query routers and servers
TODO, check with current_version
# Version 603
## Command 9D() Set parameter information 2
## Command BB() Register query
```