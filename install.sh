#!/usr/bin/env bash

set -x # Echo commands to stdout.
set -e # Exit on first error.
set -u # Treat undefined environment variables as errors.

if ! grep 'dtoverlay=dwc2' /boot/config; then
  echo "dtoverlay=dwc2" >> /boot/config.txt
fi

if ! grep dwc2 /etc/modules; then
  echo "dwc2" >> /etc/modules
fi

ENABLE_RPI_HID_PATH=/opt/enable-rpi-hid
mkdir -p "$(dirname $ENABLE_RPI_HID_PATH)"
cp enable-rpi-hid "$ENABLE_RPI_HID_PATH"
chmod +x "$ENABLE_RPI_HID_PATH"

cp usb-gadget.service /lib/systemd/system/usb-gadget.service

systemctl daemon-reload
systemctl enable usb-gadget.service

chmod +x run.sh
chmod +x pikeyboard
chmod +x ./pythonGUI/gui.py
