cargo_component_bindings::generate!();

use bindings::exports::data::transformer::component_runner::Guest;

// Bring the imported validator functions into scope
use bindings::data::validator::unit_validator;

struct Component;

// TODO: if you edit the code below e.g. println!, wasm requires std WASI interfaces to be injected by host.
impl Guest for Component {
    // Executed by host application which is executing this wasm component.
    fn run(source: String) -> f32 {
        // let ff = format!("{}:ddd",source);
        // IoC-DI for Wasm effectively
        let converted = bindings::data::transformer::message_transformers::convert_to_psi(80.0);

        let is_valid = unit_validator::validate_psi(converted);

        // Some complex logic
        if is_valid {
            converted
        }
        else {
            0.0
        }
    }
}
