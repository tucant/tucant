#!/usr/bin/python3
from dogtail.tree import root, Node

# python3 -i main.py

firefox: Node = root.application("Firefox")

def step1_open_tucant_installation_page():
    #navigation = firefox.child("Navigation", "tool bar")

    #firefox_button = navigation.child("Firefox", "button")
    #firefox_button.click()

    # https://tucant.github.io/tucant/

    #extensions_button = firefox.child("Extensions and themes", "button")
    #extensions_button.click()

    urlbar_input: Node = firefox.child(identifier="urlbar-input")
    urlbar_input.click()
    urlbar_input.keyCombo("<ctrl><a>")
    urlbar_input.typeText("https://tucant.github.io/tucant/")
    urlbar_input.keyCombo("<enter>")

# step1_open_tucant_installation_page()

def step2_install_extension():
    download_button = firefox.child("Download extension for Firefox", "link")
    download_button.click()

    firefox.child("Continue to Installation", "button").click()
    firefox.child("Add", "button").click()
    firefox.child("OK", "button").click()

#step2_install_extension()

def step3_open_tucant():
    urlbar_input: Node = firefox.child(identifier="urlbar-input")
    urlbar_input.click()
    urlbar_input.keyCombo("<ctrl><a>")
    urlbar_input.typeText("https://www.tucan.tu-darmstadt.de/")
    urlbar_input.keyCombo("<enter>")

# step3_open_tucant()

username_input: Node = firefox.child(identifier="login-username")
username_input.click()
username_input.keyCombo("<ctrl><a>")
username_input.typeText("mh58hyqa")

password_input: Node = firefox.child(identifier="login-password")
password_input.click()
password_input.keyCombo("<ctrl><a>")
password_input.typeText("mh58hyqa")

login_button: Node = firefox.child(identifier="login-button")
login_button.click()


# .text = "https://tucant.github.io/tucant/"