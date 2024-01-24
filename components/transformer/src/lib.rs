cargo_component_bindings::generate!();

use bindings::Guest;

struct Component;

// TODO: you edit the code below e.g. println!, wasm requires std WASI interfaces to be injected by host.
impl Guest for Component {
    // Executed by host application which is executing this wasm component.
    fn run(source: String) -> f32 {

        // IoC-DI for Wasm effectively
        let converted = bindings::convert_to_psi(0.2);

        // Some complex logic
        if converted > 20.0
        {
            return 20.0;
        }                

        converted
    }
}