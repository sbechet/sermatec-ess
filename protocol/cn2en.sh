#!/bin/bash
cp protocol.json protocol-en.json
cat cn2en.csv | while read l ; do cn=`echo $l | cut -d\; -f1 | tr -d \"` ; en=`echo $l | cut -d\; -f2 | tr -d \"` ; echo "$cn -> $en" ; sed -i "s|${cn}|${en}|g" protocol-en.json ; done
