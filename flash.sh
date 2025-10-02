#!/bin/bash

#loop until flash is successful

while ! espflash flash --monitor --chip esp32 -p /dev/ttyUSB0 $1; do
    echo "Flashing failed, retrying in 0.2 seconds..."
    sleep 0.2
done
