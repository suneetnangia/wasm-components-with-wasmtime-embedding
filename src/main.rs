// https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html
use anyhow::Ok;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!();

struct MyState;

// Imports into the world, like the `name` import for this world, are satisfied
// through traits.
impl TransformerImports for MyState {
    fn convert_to_psi(&mut self, bar: f32) -> std::result::Result<f32, anyhow::Error> { 
        
        // Convert Bar to PSI
        Ok(bar * 14.5038)
    }
}

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "target/wasm32-wasi/release/transformer.wasm")?;
    // let component = Component::from_file(&engine, "/home/vmadmin/repos/standalone-wasmtime-runtime/target/wasm32-wasi/release/add1.wasm")?;    

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    Transformer::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        MyState
    );

    let (bindings, _) = Transformer::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    let ff = bindings.call_run(&mut store, "Source Message")?;
    
    println!("OUTPUT: {ff}");

    Ok(())
}