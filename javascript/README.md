# vodozemac Javascript bindings

These are the Javascript bindings for [vodozemac]. Web based environments are
supported as well as node based ones.

## Build

The bindings are build using [wasm-pack]. To build Node.js compatible bindings,
simply run:

```bash

wasm-pack build --target nodejs --dev

```

Check out the supported targets using:

```bash

wasm-pack build --help

```
