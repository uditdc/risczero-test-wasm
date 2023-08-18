// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use wasm_methods::{WASM_INTERP_ELF, WASM_INTERP_ID};

fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

fn run_guest(va: i32, vb: i32) -> i32 {
    let wat = r#"
    (module
     (table 0 anyfunc)
     (memory $0 1)
     (export "memory" (memory $0))
     (export "add" (func $add))
     (func $add (; 0 ;) (param $0 i32) (param $1 i32) (result i32)
      (i32.add
       (get_local $1)
       (get_local $0)
      )
     )
    )
    "#;

    let wasm = wat2wasm(&wat).expect("Failed to parse_str");
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&wasm).unwrap())
        .add_input(&to_vec(&va).unwrap())
        .add_input(&to_vec(&vb).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, WASM_INTERP_ELF).unwrap();
    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(WASM_INTERP_ID).unwrap();

    let result: i32 = from_slice(&receipt.journal).unwrap();
    
    println!(
        "Receipt: {:?} {:?}",
        result,
        &receipt.get_metadata().unwrap()
    );

    result
}

fn main() {
    let a: i32 = 10;
    let b: i32 = 10;
    let _ = run_guest(a, b);
}

#[cfg(test)]
mod tests {
    fn wasm_add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn wasm_fn_test() {
        let a: i32 = 10;
        let b: i32 = 10;
        let result = super::run_guest(a, b);
        assert_eq!(
            result,
            wasm_add(a, b),
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}
