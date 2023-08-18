#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use wasmi::{Engine, Linker, Module, Store};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let engine = Engine::default();
    
    type WasmParams = (i32, i32);
    type WasmResult = i32;

    let wasm: Vec<u8> = env::read();
    let wasm_name = r#"add"#;
    let va: WasmParams = (env::read(), env::read());

    // Derived from the wasmi example: https://docs.rs/wasmi/0.29.0/wasmi/#example
    let module = Module::new(&engine, &mut &wasm[..]).expect("Failed to create module");
    type HostState = u32;

    let linker = <Linker<HostState>>::new(&engine);
    let mut store = Store::new(&engine, 42);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to instantiate")
        .start(&mut store)
        .expect("Failed to start");

    let wasm_fn = instance
        .get_typed_func::<WasmParams, WasmResult>(&store, &wasm_name)
        .expect("Failed to get typed_func");
    
    let res = wasm_fn.call(&mut store, va).expect("Failed to call");
    env::log(&format!("Compile WASM {} - {:?}- {}", wasm_name, va, res));
    env::commit(&res);
}
