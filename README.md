# TrueGRF YAML to GRF compiler (CLI and NPM)

TrueGRF is a project to make it more accessible for anyone to create NewGRFs for [OpenTTD](https://www.openttd.org).
For more details on the project, please visit [here](https://github.com/TrueGRF/TrueGRF).

This repository is the Rust part of the project, which offers a YAML to GRF compiler, both for the stand-alone tool and as npm library (via WASM).

## Installation / usage

### CLI

```bash
cargo run --release -- --help
```

### WASM

Have latest rust installed, and install `wasm-pack` (with `cargo install wasm-pack`).

```bash
wasm-pack build --release --target web
```

This gives you the required WASM files in the `pkg` folder.
Check `wasm-pack` how to use it in your own project.

PS: in some cases you want to replace in `pkg/package.json` the `module` key with `main` and add `"type": "module"`.

```bash
sed -i 's/"module": "truegrf.js",/"main": "truegrf.js",\n  "type": "module",/' pkg/package.json
```

## Current support

Currently TrueGRF only supports a small selection of the [GRF specifications](https://newgrf-specs.tt-wiki.net/wiki/Main_Page):

- Cargoes
- Industries
- Industry tiles

All other features are not (yet) supported.

## Filesystem layout

In order for TrueGRF to compile the YAML to GRF, the YAML files need to be in a specific structure.

- `truegrf.yaml`: the main file, to indicate it is a TrueGRF project.
- `cargoes/<name>.yaml`: one cargo definition.
- `industries/<name>.yaml`: one industry definition.

Sprites can refer to PNG files based from the root-folder.

The specific structure of the YAML can be deduced from the [source files](src/grf/mod.rs).
