//! Example of instantiating two modules which link to each other.

// You can execute this example with `cargo run` or `yarn test`

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

fn main() -> Result<()> {
    let engine = Engine::default();
    let store = Store::new(&engine);

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&store);
    let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    wasi.add_to_linker(&mut linker)?;


    // Load and compile our two modules
    let a = Module::from_file(&engine, "build/release/a.wat")?;
    let b = Module::from_file(&engine, "build/release/b.wat")?;

    // Instantiate our first module which only uses WASI, then register that
    // instance with the linker since the next linking will use it.
    let b = linker.instantiate(&b)?;
    linker.instance("b", &b)?;

    // And with that we can perform the final link and the execute the module.
    let mod_: Instance = linker.instantiate(&a)?;
    let run = mod_.get_func("run").unwrap();
    // run.get0::<>
    let run = run.get0::<i32>()?;
    let res = run()?;
    println!("{}", res);
    Ok(())
}
