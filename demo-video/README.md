```
sudo dnf install gnome-ponytail-daemon python3-gnome-ponytail-daemon
# https://github.com/pygobject/pygobject-stubs/blob/master/src/gi-stubs/repository/Atspi.pyi not in release yet
pip install dogtail git+https://github.com/pygobject/pygobject-stubs  --no-cache-dir --config-settings=config=Atspi
gsettings set org.gnome.desktop.interface toolkit-accessibility true

Enable Introspect support in Looking Glass. Press `Alt+F2`, type `lg` and
press enter to open Looking Glass. Finally enable unsafe mode:
>>> global.context.unsafe_mode = true

Fix Settings -> Accessibility -> Typing -> Repeat Keys -> Delay to the default value

setsid firefox -P tmp
sniff # show tree
python3 -i main.py
```

https://modehnal.github.io/ The main queries you will be using



https://fedoramagazine.org/automation-through-accessibility/


# Kdenlive

