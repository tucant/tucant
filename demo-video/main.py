#!/usr/bin/python3
from dogtail.tree import root, Node

# set firefox as always on top window?

firefox = root.application("Firefox")

navigation = firefox.child("Navigation", "tool bar")

#firefox_button = navigation.child("Firefox", "button")
#firefox_button.click()

# https://tucant.github.io/tucant/

#extensions_button = firefox.child("Extensions and themes", "button")
#extensions_button.click()



urlbar_input: Node = navigation.child(identifier="urlbar-input")
urlbar_input.click()
urlbar_input.keyCombo("<ctrl><a>")
urlbar_input.typeText("https://tucant.github.io/tucant/")
urlbar_input.keyCombo("<enter>")

print(urlbar_input.dump())

# .text = "https://tucant.github.io/tucant/"