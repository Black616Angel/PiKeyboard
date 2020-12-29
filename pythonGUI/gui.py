#!/usr/bin/env python3
import tkinter as tk
import json 
from pathlib import Path
import subprocess
import sys

class Btn:
	keys = ""
	def __init__(self, jsonBtn, cframe):
		self.keys= jsonBtn["keys"]
		#create the button in a grid
		tk.Button(cframe, text=jsonBtn["name"], height=2, command=self.write_string).grid(column=jsonBtn["positionx"], row=jsonBtn["positiony"], sticky=tk.W+tk.E)
	
	def write_string(self):
		ret = subprocess.run([".././pikeyboard", self.keys])
		return ret

class Keyboard:
	jsonKB = ""
	def __init__(self, jsonKB, mframe):
		self.jsonKB = jsonKB
		tk.Button(mframe, text=jsonKB["name"], command=self.fill_container).pack()

	def fill_container(self):
		#empty content frame
		for widget in cframe.winfo_children():
			widget.destroy()
		#add buttons
		for but in self.jsonKB["keys"]:
			Btn(but, cframe)

#build window
root = tk.Tk()

#menu frame
mframe = tk.Frame(root)
mframe.pack(anchor="w") #west 
#content frame
cframe = tk.Frame(root)
cframe.pack(anchor="e") #east

path = ""
if len(sys.argv) >= 1:
	path = str(sys.argv[1])
else:
	path = "../keyboards/"

kb_folder = Path(path).rglob('*.json')
for file in kb_folder:
	jsonKB = json.loads(open(file, "r").read())
	Keyboard(jsonKB, mframe)

root.mainloop()
