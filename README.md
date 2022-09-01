# smas
Stoichiometric MAtrix Solver

To build this project, you'll first need to install rust. You can accomplish this using the [rustup tool](https://www.rust-lang.org/tools/install).

Once you have rust installed, you can build smas by running cargo:

```
git clone https://github.com/TravisWheelerLab/smas/
cd smas
cargo build
```

With smas built, you can run smas with cargo:

```
# arguments after -- are passed to smas as command line arguments
cargo run -- help
cargo run -- solve ./resources/astd015.txt
```  

Alternatively you can find the binary that cargo built and run it directly:

```
./target/debug/smas solve ./resources/astd015.txt
```

## wasm api

Along with the binary veresion of the tool, smas has a web assembly API that can be built using [wasm-pack](https://github.com/rustwasm/wasm-pack), which can be installed by following [these instructions](https://rustwasm.github.io/wasm-pack/installer/)

Once you have wasm-pack installed, you can run:

```
wasm-pack build
```

This will build the wasm API as an [npm](https://www.npmjs.com/) package in the `pkg` directory.
If you have npm [installed](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm), the wasm package may be locally included into an npm project:

```
mkdir web-project
cd web-project
npm init
npm install <path to smas root directory>/pkg
```
