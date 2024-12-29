# tucant

## How does it work

This software consists of the tucan-connector component that extracts information from the html of [TUCaN](https://www.tucan.tu-darmstadt.de) and provides it as a nicer to use programming API. The tucan-injector component can then be used to show that data with a nicer UI that is written using the Rust frontend library [Yew](https://yew.rs/) and that is compiled to [WebAssembly](https://webassembly.org/). This WebAssembly can be injected into the actual TUCaN website using an extension. Then, some pages provide an overlay with the information in a nicer format and caching.

## Features

Currently, the following TUCaN pages have a nicer UI and caching:
- Veranstaltungen -> Anmeldung
  ![Veranstaltungen -> Anmeldung submenu with nicer UI](./.github/veranstaltungen_anmeldung.png)
  ![Veranstaltungen -> Anmeldung modules and courses with nicer UI](./.github/veranstaltungen_anmeldung_2.png)

## Usage

Install the extension in ./tucant-extension

## Development

### Tauri

```
cd tucant-tauri
WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri dev

WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo tauri build

cd tucant-tauri
ANDROID_HOME=~/Android/Sdk NDK_HOME=~/Android/Sdk/ndk/28.0.12674087/ cargo tauri android init

ANDROID_HOME=~/Android/Sdk NDK_HOME=~/Android/Sdk/ndk/28.0.12674087/ cargo tauri --verbose android dev

ANDROID_HOME=~/Android/Sdk NDK_HOME=~/Android/Sdk/ndk/28.0.12674087/ cargo tauri --verbose android build --debug --target aarch64

ANDROID_HOME=~/Android/Sdk NDK_HOME=~/Android/Sdk/ndk/28.0.12674087/ cargo tauri --verbose android build --apk --target aarch64

adb install /home/moritz/Documents/tucant/tucant-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk

adb install ./gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
adb shell run-as de.selfmade4u.tucant logcat
```

## Reproducibility

```
nix build .#extension
nix build --rebuild --keep-failed .#extension
```