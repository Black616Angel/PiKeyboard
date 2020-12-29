#!/usr/bin/env bash
if ! test "/dev/hidg0"; then
	sudo ./install.sh
fi

./pythonGUI/./gui.py "./keyboards/"
