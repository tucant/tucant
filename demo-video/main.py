#!/usr/bin/python3
from dogtail.tree import root
from time import sleep

# Load application root to variable.
shell = root.application("gnome-shell")

# Search the application tree for objects.
system_menu = shell.child("System", "menu")
# Click it.
system_menu.click()