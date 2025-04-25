wit_bindgen::generate!({
    world: "adder",
});

struct Adder;

impl Guest for Adder {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

export!(Adder);
