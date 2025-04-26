import time
from wasmtime import Store

# The magic, refer to https://github.com/bytecodealliance/wasmtime-py?tab=readme-ov-file#usage
import wasmtime.loader  # noqa: F401

import adder  # type: ignore


def run_adder_rs_guest():
    store = Store()
    adder_component_instance = adder.Root(store)

    start = time.time()
    result = adder_component_instance.add(store, 1, 2)
    print(f"{__name__}: 1 + 2 = {result}")
    print(f"add: took {(time.time() - start) * 1000_000:0.2f}µs")
    assert result == 3

    start = time.time()
    inp = 1000_000
    result = adder_component_instance.bench(store, inp)
    print(f"{__name__}: bench({inp}) = {result}")
    print(f"bench: took {(time.time() - start) * 1000_000:0.2f}µs")

    start = time.time()
    result = adder_component_instance.add(store, 1, 2)
    print(f"{__name__}: 1 + 2 = {result}")
    print(f"add: took {(time.time() - start) * 1000_000:0.2f}µs")
    assert result == 3


if __name__ == "__main__":
    run_adder_rs_guest()
