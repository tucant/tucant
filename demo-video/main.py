#!/usr/bin/python3
from dogtail.tree import root

# set firefox as always on top window?

firefox = root.application("Firefox")

navigation = firefox.child("Navigation", "tool bar")

#firefox_button = navigation.child("Firefox", "button")
#firefox_button.click()

# https://tucant.github.io/tucant/

#extensions_button = firefox.child("Extensions and themes", "button")
#extensions_button.click()



urlbar_input = navigation.child(identifier="urlbar-input")
#urlbar_input.doActionNamed("activate")
urlbar_input.click()
urlbar_input.typeText("https://tucant.github.io/tucant/")
urlbar_input.click()

print(urlbar_input.dump())

# .text = "https://tucant.github.io/tucant/"