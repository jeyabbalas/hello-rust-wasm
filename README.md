# Hello World in Rust + Wasm

A simple hello world function written in Rust that returns a String, a high-level data type in WebAssembly. The String contains the current UTC time. The function is compiled to Wasm binary using wasm-pack. The Wasm function is then called from JavaScript embedded into a HTML page.

To build this project, clone this project, then build the project by moving into the project directory and running the following commands in the Terminal.

```shell
> cargo build
> wasm-pack build --target web
```

Then run the project using ```npx```.

```shell
> npx serve .
```

View webpage at http://localhost:5000.