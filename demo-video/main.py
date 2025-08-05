#!/usr/bin/python3
from dogtail.tree import root

firefox = root.application("Firefox")

navigation = firefox.child("Navigation", "tool bar")

firefox_button = navigation.child("Firefox", "button")
firefox_button.click()

extensions_button = firefox.child("Extensions and themes", "button")
extensions_button.click()