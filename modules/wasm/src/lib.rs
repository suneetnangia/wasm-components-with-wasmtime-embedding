// Use wit_bindgen to generate the bindings from the component model to Rust.
// For more information see: https://github.com/bytecodealliance/wit-bindgen/
wit_bindgen::generate!({
    path: "",
    world: "transformer",
    exports: {
        world: Module,
    },
});

struct Module;

impl Guest for Module {
    fn run(a: i32, b: i32) -> i32 {
        let retVal: i32 = add(a, b);
        return retVal;
    }
}