#!/bin/bash

#loop until flash is successful

while ! espflash flash --monitor --chip esp32 -B 2100000 -p /dev/ttyUSB0 --partition-table partitions.csv $1; do
    echo "Flashing failed, retrying..."
done
