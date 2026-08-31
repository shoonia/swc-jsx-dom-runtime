# swc-jsx-dom-runtime

An experimental SWC plugin that transforms JSX into DOM runtime calls.

It is an SWC-based alternative to the Babel.js plugin provided by
[jsx-dom-runtime](https://github.com/shoonia/jsx-dom-runtime). Use this plugin
instead of the Babel.js transform when compiling JSX with SWC.

## Installation

```sh
npm install --save-dev swc-jsx-dom-runtime jsx-dom-runtime

# or
yarn add --dev swc-jsx-dom-runtime jsx-dom-runtime

# or
pnpm add --save-dev swc-jsx-dom-runtime jsx-dom-runtime
```

`jsx-dom-runtime` is required because the transformed output imports its DOM
runtime helpers.

## Configuration

For TypeScript and TSX, add the plugin and enable the TypeScript parser in
`.swcrc`:

```json
{
  "$schema": "https://swc.rs/schema.json",
  "jsc": {
    "parser": {
      "syntax": "typescript",
      "tsx": true
    },
    "experimental": {
      "plugins": [["swc-jsx-dom-runtime", {}]]
    }
  }
}
```

For JavaScript and JSX, use the ECMAScript parser instead:

```json
{
  "$schema": "https://swc.rs/schema.json",
  "jsc": {
    "parser": {
      "syntax": "ecmascript",
      "jsx": true
    },
    "experimental": {
      "plugins": [["swc-jsx-dom-runtime", {}]]
    }
  }
}
```

Do not configure `jsc.transform.react`; this plugin performs the JSX
transformation.

The package contains a precompiled WASI WebAssembly plugin. Your SWC version
must support the plugin ABI used by this package.

## Development

The `wasm32-wasip1` Rust target is required.

```sh
rustup target add wasm32-wasip1
npm test
npm run build
```

To inspect exactly what npm will publish:

```sh
npm pack --dry-run
```

## License

MIT
