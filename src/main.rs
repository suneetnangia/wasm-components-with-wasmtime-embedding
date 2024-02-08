use wasmtime_wasi::preview2;
// https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html
use anyhow::Ok;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!();

// Imports into the world, like the `name` import for this world, are satisfied
// through traits.
impl data::transformer::message_transformers::Host for ServerWasiView {
    fn convert_to_psi(&mut self, bar: f32) -> std::result::Result<f32, anyhow::Error> {
        // Convert Bar to PSI
        Ok(bar * 14.5038)
    }

    fn convert_to_bar(&mut self, psi: f32) -> std::result::Result<f32, anyhow::Error> {
        // Convert PSI to Bar
        Ok(psi * 14.5038)
    }
}

fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    // config.async_support(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "composed.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    Transformer::add_to_linker(&mut linker, |state: &mut ServerWasiView| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let wasi_view = ServerWasiView::new();
    let mut store = Store::new(&engine, wasi_view);

    let (bindings, _) = Transformer::instantiate(&mut store, &component, &linker)?;
    let output = bindings.interface0.call_run(&mut store, "")?;

    println!("OUTPUT: {output}");

    Ok(())
}
struct ServerWasiView {
    table: Table,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = Table::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();

        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
