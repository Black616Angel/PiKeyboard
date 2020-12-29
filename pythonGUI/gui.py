#!/usr/bin/env python3
import tkinter as tk
import json 
from pathlib import Path
import subprocess

def add_menu_button(jsonKB, mframe):
	Button(mframe, text=jsonKB["name"], command=build_container(jsonKB)).pack()

def build_button(jsonBut, cframe):
	Button(cframe, text=jsonBut["name"], command=write_string(jsonBut["keys"])).pack()

def write_string(str):
	ret = subprocess.run(["./pikeyboard", str])
	return ret

def fill_container(jsonKB):
	for but in jsonKB[""]:
		build_button(but)

root = tk.Tk()
all_kb = []
mframe = tk.Frame(root)
cframe = tk.Frame(root)

kb_folder = Path('./keyboards/').rglob('*.json')
for file in kb_folder:
	jsonKB = json.loads(open(file, "r").read())
	all_kb.append(jsonKB)
	add_menu_button(jsonKB, mframe)

root.mainloop()