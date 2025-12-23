# RustyNote 

RustyNote is a simple browser-based Markdown editor powered by Rust and Leptos. The app runs as WASM in the browser and is built with Trunk.

## Demo

https://dr-eckig.github.io/RustyNote/

## Features

- Live preview in split mode
- Markdown formatting buttons and shortcuts
- GitHub-Flavored Markdown (GFM) support
- Copy and downloading the markdown content
- Line numbers in the editor
- Theme switcher (light-/ darkmode)
- Integrated documentation

## Setup

**Requirements**:
- Rust (stable recommended)
- wasm32 target
- Trunk

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Local dev server:
```bash
trunk serve
```

Release build (e.g. for GitHub Pages):
```bash
trunk build --release --public-url "/markdown/"
```
