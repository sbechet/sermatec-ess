# Proxmox 6.4 Wifi Network configuration

## /etc/sysctl.conf

Add `net.ipv4.ip_forward=1`

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


# Proxmox 8 Wifi Network configuration

## /etc/sysctl.conf

Add `net.ipv4.ip_forward=1`

### IWD Configuration

```
apt install iwd
systemctl --now enable iwd
iwctl
[iwd]# device list
[iwd]# device wlan0 scan
[iwd]# station wlan0 get-networks
[iwd]# station wlan0 connect $MYNETWORK
$MYPASSWORD
[iwd]# device list
[iwd]# exit
```

### dhclient.conf configuration

```
request subnet-mask, broadcast-address, time-offset,
       host-name,
       interface-mtu,
       rfc3442-classless-static-routes, ntp-servers;

supersede domain-name "localdomain";
supersede domain-name-servers $IP
```

## /etc/network/interfaces

add to interfaces configuration:

```
auto wlan0
iface wlan0 inet dhcp
        post-up   iptables -t nat -A POSTROUTING -s '192.168.0.0/24' -o wlan0 -j MASQUERADE
        post-down iptables -t nat -D POSTROUTING -s '192.168.0.0/24' -o wlan0 -j MASQUERADE
```


Then `ifup wlan0`
