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

# TODO clear browser data before so we're logged out?
# TODO uninstall extension before

def toggle_navigation():
    # TODO maybe we can check whether it is expanded in accessibility info
    return
    # on mobile
    #firefox.child("Toggle navigation", "button").click()

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

def step2_install_extension():
    sleep(2)
    download_button = firefox.child("Download extension for Firefox", "link")
    download_button.click()

    sleep(0.5)
    firefox.child("Continue to Installation", "button").click()
    firefox.child("Add", "button").click()
    firefox.child("OK", "button").click()

def step2_5_extension_settings():
    firefox.child("Extensions", "button").click()
    sleep(1)
    #firefox.dump(fileName="test")
    firefox.child("TUCaN't", "button").click()
    firefox.child("Go to options", "button").click()
    firefox.child("Anonymize grades (for demoing).", "check box").click()

def step3_open_tucant():
    urlbar_input: Node = firefox.child(identifier="urlbar-input")
    urlbar_input.click()
    urlbar_input.keyCombo("<ctrl><a>")
    urlbar_input.typeText("https://www.tucan.tu-darmstadt.de/")
    urlbar_input.keyCombo("<enter>")


def step4_login():
    toggle_navigation()

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


def step5_aktuelles():
    # on mobile (TODO I think login should close navbar if it does not)
    toggle_navigation()

    aktuelles_button = firefox.child("Aktuelles", "button")
    aktuelles_button.scroll_to(SCROLL_ANYWHERE)
    aktuelles_button.click()
    firefox.child("Aktuelles", "link").click()


def step6_vv():
    toggle_navigation()

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
    # TODO maybe we can improve this scrolling
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


def step7_semestermodule():
    toggle_navigation()
    firefox.child("Veranstaltungen", "button").click()
    firefox.child("Meine Semestermodule", "link").click()
    sleep(3)
    firefox.child("Select semester", "combo box").click()
    firefox.child("WiSe 2024/25", "menu item").click()
    sleep(3)

def step8_veranstaltungen():
    toggle_navigation()
    firefox.child("Veranstaltungen", "button").click()
    firefox.child("Meine Veranstaltungen", "link").click()
    sleep(3)
    firefox.child("Select semester", "combo box").click()
    firefox.child("WiSe 2024/25", "menu item").click()
    sleep(3)

def step9_anmeldung_und_pruefungen():
    toggle_navigation()
    firefox.child("Veranstaltungen", "button").click()
    firefox.child("Anmeldung", "link").click()

    informatik_link: Atspi.Component | Node = firefox.child("Vertiefungen, Wahlbereiche und Studium Generale", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    informatik_link: Atspi.Component | Node = firefox.child("Vertiefungen", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    informatik_link: Atspi.Component | Node = firefox.child("Fachprüfungen aus den Basis Wahlbereichen und Wahlbereichen der Individuellen Vertiefung", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    informatik_link: Atspi.Component | Node = firefox.child("Basis Wahlbereiche", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    informatik_link: Atspi.Component | Node = firefox.child("Theorie (Theoretische Informatik)", "link")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    # seamless interop between tucant and tucan
    informatik_link: Atspi.Component | Node = firefox.child("Zum Modul anmelden", "button")
    informatik_link.scroll_to(SCROLL_ANYWHERE)
    sleep(0.5) # scrolling seems to have a delay
    informatik_link.click()

    sleep(2)

    # seamless interop between tucan and tucant
    firefox.child("Prüfungen", "link").click()
    firefox.child("Meine Prüfungen", "link").click()

    # if you want you can still open in tucan
    firefox.child("TUCaN't", "document web").click(3) # right click somewhere on page
    firefox.child("TUCaN't", "menu").click()
    firefox.child("Open page in TUCaN", "menu item").click()
    # Open page in TUCaN in new tab

    sleep(2)

    firefox.child("Technische Universität Darmstadt", "document web").click(3) # right click somewhere on page
    firefox.child("TUCaN't", "menu").click()
    firefox.child("Open page in TUCaN't in new tab", "menu item").click()

    sleep(1)

def step10_ergebnisse():
    sleep(2)
    # TODO I think these are broken when coming from a page that was open in tucan?
    toggle_navigation()
    firefox.child("Prüfungen", "button").click()
    firefox.child("Modulergebnisse", "link").click()
    sleep(5)

    toggle_navigation()
    firefox.child("Prüfungen", "button").click()
    firefox.child("Prüfungsergebnisse", "link").click()
    sleep(5)

    toggle_navigation()
    firefox.child("Prüfungen", "button").click()
    firefox.child("Leistungsspiegel", "link").click()
    sleep(5)
    firefox.child("Select course of study", "combo box").click()
    firefox.child("B.Sc. Informatik (2015)", "menu item").click()

sleep(3)
step1_open_tucant_installation_page()
step2_install_extension()
step2_5_extension_settings()
step3_open_tucant()
step4_login()
step5_aktuelles()
step6_vv()
step7_semestermodule()
step8_veranstaltungen()
step9_anmeldung_und_pruefungen()
step10_ergebnisse()