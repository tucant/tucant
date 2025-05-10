# tucant

The Campus-Management System of TU Darmstadt called TUCaN lacks quality and usability. Therefore this extension adds some quality of life improvements.

* "Veranstaltungen -> Anmeldung" caches pages so repeatedly navigating to the same page is extremely fast and automatically descends into menus with only one entry. It's UI looks the following way:
  ![Veranstaltungen -> Anmeldung submenu with nicer UI](./.forgejo/veranstaltungen_anmeldung.png)
  ![Veranstaltungen -> Anmeldung modules and courses with nicer UI](./.forgejo/veranstaltungen_anmeldung_2.png)
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
rustup component add --toolchain nightly-2025-04-02 rust-src rustc-dev llvm-tools-preview
rustup component remove --toolchain nightly-2025-04-02 rustfmt
cargo +nightly-2025-04-02 install --force --git https://github.com/tucant/rustfmt --branch html-extractor-formatting rustfmt-nightly
cargo +nightly-2025-04-02 fmt
```

### Running as local webserver

```bash
cd crates/tucant-yew/
mkdir ../../tucant-extension/dist
trunk serve --features api

# in second tab
bacon tucant-api

# http://localhost:1420/#/
```

### Developing the extension

```
cd tucant-extension/
./watch.sh
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

This software consists of the tucan-connector component that extracts information from the html of [TUCaN](https://www.tucan.tu-darmstadt.de) and provides it as a nicer to use programming API. The tucan-injector component can then be used to show that data with a nicer UI that is written using the Rust frontend library [Yew](https://yew.rs/) and that is compiled to [WebAssembly](https://webassembly.org/). This WebAssembly can be injected into the actual TUCaN website using an extension. Then, some pages provide an overlay with the information in a nicer format and caching.

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

https://developer.chrome.com/blog/wasm-debugging-2020

DevTools Experiments -> WebAssembly Debugging

Does not work for the extension, only the local api server.
[C/C++ DevTools Support (DWARF)] Failed to load debug symbols for chrome-extension://jdmjpehgmiafdnhmoambipgghlodiagm/dist/tucant-yew-c6bc98f3c4dab7fd_bg.wasm (TypeError: Failed to fetch)

https://github.com/ChromeDevTools/devtools-frontend/blob/main/extensions/cxx_debugging/src/manifest.json.in

https://chromium.googlesource.com/devtools/devtools-frontend/+/main/docs/get_the_code.md

```
git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
export PATH=~/Documents/depot_tools/:$PATH
mkdir devtools
cd devtools
fetch devtools-frontend
nano .gclient
solutions = [
 {
   "name"        : "devtools-frontend",
   "url"         : "https://chromium.googlesource.com/devtools/devtools-frontend",
   "deps_file"   : "DEPS",
   "managed"     : True,
   "custom_deps" : {
   },
   "custom_vars": {
     "checkout_cxx_debugging_extension_deps": True
   },
 }
]
gclient sync
cd devtools-frontend/extensions/cxx_debugging
./tools/bootstrap.py -debug ../../out
ls ../../out/DevTools_CXX_Debugging.stage2/src/
```

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
