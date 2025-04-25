use wasmtime::Engine;
use wasmtime::Result;
use wasmtime::component::bindgen;
use wasmtime_wasi::add_to_linker_sync;

bindgen!({
    path: "./wit/adder.wit",
    world: "adder",
});

pub fn run_adder_sync(engine: &Engine) -> Result<()> {
    println!("Loading guest-adder-py, will take dozens of seconds");
    let (component, mut linker, mut store) =
        get_component_linker_store(engine, "./guest_adder_py.wasm", "./guest_adder_py.wasm")?;
    add_to_linker_sync(&mut linker)?;
    let bindings = Adder::instantiate(&mut store, &component, &linker)?;
    let a = 1;
    let b = 2;
    let result = bindings.call_add(&mut store, a, b)?;
    println!("py_wasm: {} + {} = {}", a, b, result);
    Ok(())
}

fn main() {
    let engine = Engine::default();
    run_adder_sync(&engine).unwrap();
}

use wasmtime::Store;
use wasmtime::component::Component;
use wasmtime::component::Linker;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::IoImpl;
use wasmtime_wasi::IoView;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiImpl;
use wasmtime_wasi::WasiView;

struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of WasiView and IoView
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl IoView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl ComponentRunStates {
    pub fn new() -> Self {
        ComponentRunStates {
            wasi_ctx: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        }
    }
}

/// Copied from [wasmtime_wasi::io_type_annotate]
pub fn io_type_annotate<T: IoView, F>(val: F) -> F
where
    F: Fn(&mut T) -> IoImpl<&mut T>,
{
    val
}

/// Copied from [wasmtime_wasi::type_annotate]
pub fn type_annotate<T: WasiView, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiImpl<&mut T>,
{
    val
}

fn get_component_linker_store(
    engine: &Engine,
    path: &'static str,
    alt_path: &'static str,
) -> Result<(Component, Linker<ComponentRunStates>, Store<ComponentRunStates>)> {
    let component =
        Component::from_file(engine, path).or_else(|_| Component::from_file(&engine, alt_path))?;
    let linker = Linker::new(&engine);
    let state = ComponentRunStates::new();
    let store = Store::new(&engine, state);
    Ok((component, linker, store))
}
