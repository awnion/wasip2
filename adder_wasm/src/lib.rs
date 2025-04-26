wit_bindgen::generate!({
    world: "adder",
    path: "../wit/adder.wit",
});

struct Adder;

impl Guest for Adder {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn bench(a: i32) -> i32 {
        let mut sum = 0;
        for _ in 0..a {
            sum += a;
        }
        sum
    }
}

export!(Adder);
