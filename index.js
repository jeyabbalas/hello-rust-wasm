import wasmInit, {
    greet
  } from "./pkg/hello_rust_wasm.js";

  const runWasm = async () => {
    const rustWasm = await wasmInit("./pkg/hello_rust_wasm_bg.wasm");
    const greetings = greet();
    console.log(greetings);
  };
  runWasm();