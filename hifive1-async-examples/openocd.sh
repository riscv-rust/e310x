#!/bin/bash

# This script runs OpenOCD with the specified configuration file
# for the HiFive1 board. The configuration file is selected based
# on the revb argument, which is a boolean flag that indicates
# whether the HiFive1 Rev B board is being used. If the revb
# argument is not provided, the default configuration file for
# the HiFive1 Rev A board is used.

# Default path to OpenOCD
OPENOCD_PATH="openocd"
REVB=false

# Parse command-line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -p|--path) OPENOCD_PATH="$2"; shift ;;
        revb) REVB=true ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Determine the configuration file based on the revb argument
if [ "$REVB" = true ]; then
    CONFIG_FILE="sifive-hifive1-revb.cfg"
else
    CONFIG_FILE="sifive-hifive1.cfg"
fi

# Run OpenOCD with the specified configuration file
echo "Running $OPENOCD_PATH -f board/$CONFIG_FILE"
$OPENOCD_PATH -f board/$CONFIG_FILE
