package wat

import (
	"fmt"
	"log"

	"github.com/bytecodealliance/wasmtime-go"
)

func add(a int32, b int32) int32 {
	fmt.Println("Callback for function 'add'...")
	return a + b
}

func ProcessWat() {
	// Configure the initial compilation environment.
	engine := wasmtime.NewEngine()

	module, err := wasmtime.NewModuleFromFile(engine, "../../target/wasm32-wasi/release/composedtransformer.wasm")
	if err != nil {
		log.Fatal(err)
	}

	// Configure the initial compilation environment.
	store := wasmtime.NewStore(engine)

	// Here we handle the imports of the module, which in this case is our
	// `helloFunc` callback.
	addCallback := wasmtime.WrapFunc(store, add)

	// Once we've got that all set up we can then move to the instantiation
	// phase, pairing together a compiled module as well as a set of imports.
	// Note that this is where the wasm `start` function, if any, would run.
	instance, err := wasmtime.NewInstance(store, module, []wasmtime.AsExtern{addCallback})
	if err != nil {
		log.Fatal(err)
	}

	// Next we poke around a bit to extract the `run` function from the module.
	run := instance.GetFunc(store, "run")
	if run == nil {
		log.Fatal("Failed to find function export `run`")
	}

	// And last but not least we can call it!
	fmt.Println("Calling export...")
	result, err := run.Call(store, 1, 2)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Result of `run` function:", result)

}
