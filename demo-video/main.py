#!/usr/bin/python3
import os
import gi

gi.require_version('Atspi', '2.0')
from gi.repository import Atspi

from dogtail.tree import root, Node
from dotenv import load_dotenv
from time import sleep

from pyatspi import SCROLL_ANYWHERE

load_dotenv()

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

#step1_open_tucant_installation_page()

def step2_install_extension():
    download_button = firefox.child("Download extension for Firefox", "link")
    sleep(0.5)
    download_button.click()

    sleep(0.5)
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

step3_open_tucant()

def step4_login():
    # on mobile
    firefox.child("Toggle navigation", "button").click()

    # if we're already logged in this fails
    username_input: Node = firefox.child(identifier="login-username")
    username_input.click()
    username_input.keyCombo("<ctrl><a>")
    username_input.typeText(os.getenv("TUCAN_USERNAME"))

    password_input: Node = firefox.child(identifier="login-password")
    password_input.click()
    password_input.keyCombo("<ctrl><a>")
    password_input.typeText(os.getenv("TUCAN_PASSWORD"))

    login_button: Node = firefox.child(identifier="login-button")
    login_button.click()

#step4_login()

def step5_aktuelles():
    # on mobile (TODO I think login should close navbar if it does not)
    firefox.child("Toggle navigation", "button").click()

    aktuelles_button = firefox.child("Aktuelles", "button")
    aktuelles_button.scroll_to(SCROLL_ANYWHERE)
    aktuelles_button.click()
    firefox.child("Aktuelles", "link").click()

# step5_aktuelles()

def step6_vv():
    # on mobile
    firefox.child("Toggle navigation", "button").click()

    firefox.child("VV", "button").click()
    firefox.child("Vorlesungsverzeichnis", "link").click()

    informatik_link: Atspi.Component | Node = firefox.child("FB20 - Informatik", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    pflichtveranstaltungen: Atspi.Component | Node = firefox.child("Pflichtveranstaltungen", "link")
    pflichtveranstaltungen.click()

    aud: Atspi.Component | Node = firefox.child("Kurs 20-00-0005-iv Algorithmen und Datenstrukturen", "link")
    aud.click()

    aud: Atspi.Component | Node = firefox.child("Algorithmen und Datenstrukturen 01", "link")
    aud.click()
    aud.doActionNamed("jump") # click does not work on mobile mode as it clicks at the wrong place
    sleep(3)

    # https://gitlab.gnome.org/GNOME/gtk/-/blob/main/gdk/gdkkeysyms.h
    # https://github.com/vhumpa/dogtail/blob/3600ef901bcd7b4f8d64dce17a600219dcc1abf9/dogtail/rawinput.py#L477
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(1)
    firefox.keyCombo("<pagedown>")
    sleep(3)
    firefox.keyCombo("<Home>")

#step6_vv()

# on mobile
firefox.child("Toggle navigation", "button").click()

firefox.child("Veranstaltungen", "button").click()
firefox.child("Meine Semestermodule", "link").click()

firefox.child("Select semester", "combo box").click()
firefox.child("WiSe 2024/25", "menu item").click()