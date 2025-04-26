use wasmtime::AsContextMut;
use wasmtime::Engine;
use wasmtime::component::Component;
use wasmtime::component::Linker;
use wasmtime::component::bindgen;
use wasmtime_wasi::IoView;
use wasmtime_wasi::ResourceTable;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::WasiView;

bindgen!({
    world: "adder",
});

pub struct ComponentRunStates {
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

pub fn get_adder_component<T, S>(engine: &Engine, linker: &Linker<T>, store: S) -> Adder
where
    S: AsContextMut<Data = T>,
{
    let component =
        Component::from_file(engine, "target/wasm32-wasip2/release/adder_wasm.wasm").unwrap();
    let adder = Adder::instantiate(store, &component, linker).unwrap();
    adder
}
