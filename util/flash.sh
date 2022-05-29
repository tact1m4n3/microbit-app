#!/bin/bash

arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/debug/microbit-app microbit-app.hex
srec_cat \
    util/bootloader.hex -intel \
    microbit-app.hex -intel \
    -o final.hex -intel
cp -rf final.hex /Volumes/MICROBIT
rm -rf final.hex microbit-app.hex
