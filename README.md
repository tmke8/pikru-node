# pikru-node

WASM bindings for [pikru](https://crates.io/crates/pikru), a Rust library that renders [Pikchr](https://pikchr.org/) diagrams to SVG.

## Installation

```bash
npm install @tmke8/pikru
```

## Usage

```javascript
import { Pikru } from '@tmke8/pikru';

const pikru = new Pikru();
const svg = pikru.render('box "Hello"; arrow; box "World"');
```

### Options

The `Pikru` constructor accepts an optional options object:

```javascript
const pikru = new Pikru({ cssVariables: false, explicitSize: false });
```

| Option         | Type      | Default | Description                                                      |
|----------------|-----------|---------|------------------------------------------------------------------|
| `cssVariables` | `boolean` | `false` | Use CSS variables for colors, enabling light/dark mode support.  |
| `explicitSize` | `boolean` | `false` | Add explicit `width` and `height` attributes to the SVG element. |

## Development

### Building
Building requires [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen).

Build the Node.js package:

```bash
cd node
npm install
npm run build
```

Build for web (requires [wasm-opt](https://github.com/WebAssembly/binaryen) for binary size optimization):

```bash
make playground
```

### Testing

```bash
cd node
npm test
```

### Playground

A simple test page is included in the `playground/` directory:

```bash
python3 -m http.server 8000 --directory playground
```

Then open http://localhost:8000 in your browser.
