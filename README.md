# pikru-node

WASM bindings for [pikru](https://crates.io/crates/pikru), a Rust library that renders [Pikchr](https://pikchr.org/) diagrams to SVG.

## Building

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

```bash
cd pikru-wasm
wasm-pack build --target web --out-dir ../playground/pkg
```

## Usage

```javascript
import init, { render } from './pkg/pikru_wasm.js';

await init();

const svg = render('box "Hello" arrow box "World"');
```

## Playground

A simple test page is included in the `playground/` directory. To run it:

```bash
cd playground
python3 -m http.server 8000
```

Then open http://localhost:8000 in your browser.
