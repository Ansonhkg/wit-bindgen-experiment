# Getting started

## Step 1

```
cargo build --target wasm32-wasi
```

## Step 2

- Read [Creating components: WASI](https://github.com/bytecodealliance/wit-bindgen#creating-components-wasi) 

- The current wasmtime is [V13.0.0 (#7067)](https://github.com/bytecodealliance/wasmtime/releases/tag/v13.0.0)
```
wasm-tools component new ./target/wasm32-wasi/debug/your_package_name.wasm -o my-component.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```

## Step 3

```
wasm-tools component wit ./my-component.wasm
```

output

```
anson@c-2 wasm % wasm-tools component wit ./my-component.wasm

package root:component

world root {
  import wasi:io/streams
  import wasi:filesystem/types
  import wasi:filesystem/preopens
  import wasi:cli/environment
  import wasi:cli/exit
  import wasi:cli/stdin
  import wasi:cli/stdout
  import wasi:cli/stderr
  import wasi:cli/terminal-input
  import wasi:cli/terminal-output
  import wasi:cli/terminal-stdin
  import wasi:cli/terminal-stdout
  import wasi:cli/terminal-stderr
  import print: func(msg: string)

  export run: func()
}
```