use host_adder::ComponentRunStates;
use host_adder::get_adder_component;
use wasmtime::Engine;
use wasmtime::Store;
use wasmtime::component::Linker;
use wasmtime_wasi::add_to_linker_sync;

fn main() {
    let engine = Engine::default();
    let mut linker: Linker<ComponentRunStates> = Linker::new(&engine);
    add_to_linker_sync(&mut linker).unwrap();

    let mut store = {
        let state = ComponentRunStates::new();
        Store::new(&engine, state)
    };
    let adder = get_adder_component(&engine, &linker, &mut store);

    for i in 0..5 {
        let start = std::time::Instant::now();
        let res = adder.call_bench(&mut store, i).unwrap();
        let elapsed = start.elapsed();
        println!("call_bench Iteration {i} took: {elapsed:?}. Result: {res}");
    }

    for i in 0..5 {
        let start = std::time::Instant::now();
        let res = adder.call_add(&mut store, i, i).unwrap();
        let elapsed = start.elapsed();
        println!("call_add Iteration {i} took: {elapsed:?}. Result: {res}");
    }
}
