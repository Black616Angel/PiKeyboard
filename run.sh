#!/usr/bin/env bash
if [[ $( systemctl status usb-gadget ) ]] && test "/dev/hidg0"; then
	sudo ./install.sh
fi

./pythonGUI/./gui.py "./keyboards/"