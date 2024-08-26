# quick MMR

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)

This is a small app written for a friend to calculate MMR between his Magic the Gathering group after consecutive matches. Made partially to teach myself some basic Rust. Builds to WASM and native.

### Testing locally

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web locally

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache the app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web deploy
GH Pages will deploy using the associated action in this repo off the `gh-pages` branch.

WASM deployment at <https://astrojord.github.io/quick-mmr/>. May lag behind development branches by a few minutes.
