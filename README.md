# tucant

The Campus-Management System of TU Darmstadt called TUCaN lacks quality and usability. Therefore this extension adds some quality of life improvements.

* "Veranstaltungen -> Anmeldung" caches pages so repeatedly navigating to the same page is extremely fast and automatically descends into menus with only one entry.
* TUCaN sometimes chains redirects. In some known places the intermediate redirect is skipped by the extension to speed up the navigation. Furthermore, there are some pages that wait for 500 milliseconds before redirecting. These pages are also skipped at some known places.
* The URL contains your session ID. Therefore sharing URLs with others does not work. The extension automatically changes the session ID in the url to your session ID so sharing URLs works for users of the extension.
* An experimental mobile first design can be activated.

## Installation

Go to https://tucant.github.io/tucant/ and follow the instructions.

## Development

Requirements: [Rustup](https://www.rust-lang.org/tools/install)

Recommended: VSCodium with rust-analyzer extension

Just open the project in VSCodium.

### Running all tests

```
cargo test
```

### Running UI tests

**Important: Do NOT run in `nix develop` environment**

```
nix run .#test-dev
```

### Formatting

We use a fork of rustfmt to format our custom html extractor macro.
```
# install minimal profile
rustup toolchain install nightly-2025-04-02 --component rust-src rustc-dev llvm-tools-preview
rustup component remove --toolchain nightly-2025-04-02 rustfmt
cargo +nightly-2025-04-02 install --force --git https://github.com/tucant/rustfmt --branch html-extractor-formatting rustfmt-nightly
cargo +nightly-2025-04-02 fmt
rustup run nightly-2025-04-02 dx fmt
```

### Running as local webserver

```bash
cargo install --git https://github.com/mohe2015/dioxus --branch my dioxus-cli

cd crates/tucant-dioxus/
dx serve --platform web --features api --verbose

# in second tab
bacon tucant-api

# http://localhost:8080/#/
```

### Developing the extension

```
cd crates/tucant-dioxus/
dx bundle --platform web --out-dir ../../tucant-extension/ --base-path public --features direct --release
```

Go to Firefox Extensions, click settings, debug addons. Then click load temporary add-on and select ./tucant-extension/manifest.json
See https://extensionworkshop.com/documentation/develop/debugging/.

## Building extension (not for development)

```bash
podman build --output . .
```
This will produce a tucant-extension.zip in the current working directory.

## Packaging

### Chromium

https://developer.chrome.com/docs/extensions/how-to/distribute/host-on-linux#packaging

Open Chromium -> chrome://extensions/ -> Pack extension -> Choose folder -> Pack. Store private key in a secure place

```bash
chromium --pack-extension=tucant-extension --pack-extension-key=/path/to/tucant-extension.pem
```

### Firefox

https://extensionworkshop.com/documentation/publish/distribute-sideloading/

ZIP the extension files.

Upload to AMO as unlisted extension and pray that it gets signed quickly.

## How does it work

This software consists of the tucan-connector component that extracts information from the html of [TUCaN](https://www.tucan.tu-darmstadt.de) and provides it as a nicer to use programming API. The tucan-injector component can then be used to show that data with a nicer UI that is written using the Rust frontend library [Dioxus](https://dioxus.dev/) and that is compiled to [WebAssembly](https://webassembly.org/). This WebAssembly can be injected into the actual TUCaN website using an extension. Then, some pages provide an overlay with the information in a nicer format and caching.

## API

```
bacon tucant-api
```
http://localhost:3000/swagger-ui/

http://localhost:3000/api-docs/openapi.json

```
cargo run --bin schema > schema.json
```
https://editor-next.swagger.io/

## tucan-connector

.env
```
TUCAN_USERNAME=
TUCAN_PASSWORD=
SESSION_ID=
SESSION_KEY=
```

## Debugging

https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb

## Coverage

```
# https://doc.rust-lang.org/rustc/instrument-coverage.html#test-coverage
cd tucan-connector
RUSTFLAGS="-C instrument-coverage" cargo test
nix shell nixpkgs#llvmPackages_19.bintools-unwrapped
llvm-profdata merge *.profraw -o default.profdata

llvm-cov show -Xdemangler=/home/moritz/.cargo/bin/rustfilt /home/moritz/Documents/tucant/target/debug/deps/tucan_connector-90eac6df256ec2c3 \
    -format=html \
    -output-dir=target/coverage \
    -instr-profile=default.profdata \
    -show-line-counts-or-regions \
    -show-instantiations

xdg-open target/coverage/index.html 
```

## Android

```
sudo systemctl stop firewalld.service
adb connect 172.18.61.176:43109
adb uninstall com.example.TucantDioxus
dx serve --device --platform android --hotpatch --verbose

adb logcat -c
adb shell run-as com.example.TucantDioxus logcat

dx bundle --platform android --device
/home/moritz/Documents/tucant/target/dx/tucant-dioxus/debug/android/app/app/build/outputs/apk/debug/app-debug.apk

dx serve --platform android --hotpatch --verbose
cargo run --manifest-path /home/moritz/Documents/dioxus/packages/cli/Cargo.toml serve --platform android --verbose
# grep for RustStdoutStderr

dx bundle --platform android --release
adb install target/dx/tucant-dioxus/release/android/app/app/build/outputs/apk/release/app-release.apk


cargo run --manifest-path /home/moritz/Documents/dioxus/packages/cli/Cargo.toml build --platform android
```

## Linux

```
dx serve --platform linux --hotpatch --verbose

```

```
diff --color <(cat *_registration-N383703888296780\,-N0\,-N0\,-N0_B.Sc.\ Informatik\ \(2015\).json | jq 'sort_by(.path) | del(.[].studiumsauswahl) | del(.[].entries.[].module.registration_state) | del(.[].entries.[].courses.[].[1].registration_button_link) | del(.[].entries.[].courses.[].[1].location_or_additional_info) | del(.[].entries.[].courses.[].[1].limit_and_size)') <(cat *_registration-N376333755785484\,-N0\,-N0\,-N0_B.Sc.\ Informatik\ \(2015\).json | jq 'sort_by(.path) | del(.[].studiumsauswahl) | del(.[].entries.[].module.registration_state) | del(.[].entries.[].courses.[].[1].registration_button_link) | del(.[].entries.[].courses.[].[1].location_or_additional_info) | del(.[].entries.[].courses.[].[1].limit_and_size)' | sed 's/N376333755785484/N383703888296780/g')


oh no there are different numbers for the same PO (probably for each semester?) maybe because depending on your starting semester the modules change?
```