#!/usr/bin/env python3
import tkinter as tk
import json 
from pathlib import Path
import subprocess

class Btn:
	name = ""
	keys = ""
	def __init__(self, jsonBtn, cframe):
		print(jsonBtn["name"])
		self.name= jsonBtn["name"]
		self.keys= jsonBtn["keys"]
		tk.Button(cframe, text=self.name, command=self.write_string).grid(column=jsonBtn["positionx"], row=jsonBtn["positiony"])
	
	def write_string(self):
		print(self.keys)
		ret = subprocess.run([".././pikeyboard", self.keys])
		return ret
class Keyboard:
	jsonKB = ""
	def __init__(self, jsonKB, mframe):
		print(jsonKB["name"])
		self.jsonKB = jsonKB
		tk.Button(mframe, text=jsonKB["name"], command=self.fill_container).pack()

	def fill_container(self):
		for widget in cframe.winfo_children():
			widget.destroy()
		cframe
		for but in self.jsonKB["keys"]:
			Btn(but, cframe)

root = tk.Tk()
all_kb = []
mframe = tk.Frame(root)
mframe.pack()
cframe = tk.Frame(root)
cframe.pack()

kb_folder = Path('../keyboards/').rglob('*.json')
for file in kb_folder:
	jsonKB = json.loads(open(file, "r").read())
	all_kb.append(Keyboard(jsonKB, mframe))

root.mainloop()
