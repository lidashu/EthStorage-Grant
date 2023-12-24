use risc0_zkvm::{default_executor, ExecutorEnv};
use std::time::{SystemTime, UNIX_EPOCH};
use wasmfibo_methods::{WASM_INTERP_ELF};


fn timestamp1() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}


fn run_guest() -> i32 {
    let wasm = std::fs::read("fib_risc.wasm").unwrap(); 
    // let wasm = wat2wasm(&wat).expect("Failed to parse_str");

    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .build()
        .unwrap();

    let exec = default_executor();
    let receipt = exec.execute_elf(env, WASM_INTERP_ELF).unwrap();
    
    let result: i32 = receipt.journal.decode().unwrap();

    result
}

fn main() {
    let ts1 = timestamp1();
    println!("TimeStamp1: {}", ts1);
    let _ = run_guest();
    let ts2 = timestamp1();
    println!("TimeStamp1: {}", ts2);
}
