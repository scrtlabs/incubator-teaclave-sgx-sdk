// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![cfg_attr(not(target_vendor = "teaclave"), no_std)]
#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]

#[cfg(not(target_vendor = "teaclave"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;
use sgx_types::error::SgxStatus;
use std::env;
use std::io::{self, Write};
use std::slice;
use std::string::String;
use std::vec::Vec;

use wasmer::{
    imports, wat2wasm, Cranelift, Function, Instance, Module, NativeFunc, Store, Universal,
};

fn main() -> anyhow::Result<()> {
    // First we create a simple Wasm program to use with Wasmer.
    // We use the WebAssembly text format and use `wasmer::wat2wasm` to compile
    // it into a WebAssembly binary.
    //
    // Most WebAssembly programs come from compiling source code in a high level
    // language and will already be in the binary format.
    let wasm_bytes = wat2wasm(
        br#"
(module
  ;; First we define a type with no parameters and no results.
  (type $no_args_no_rets_t (func (param) (result)))
  ;; Then we declare that we want to import a function named "env" "say_hello" with
  ;; that type signature.
  (import "env" "say_hello" (func $say_hello (type $no_args_no_rets_t)))
  ;; Finally we create an entrypoint that calls our imported function.
  (func $run (type $no_args_no_rets_t)
    (call $say_hello))
  ;; And mark it as an exported function named "run".
  (export "run" (func $run)))
"#,
    )?;

    // Next we create the `Store`, the top level type in the Wasmer API.
    //
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    //
    // However for the purposes of showing what's happening, we create a compiler
    // (`Cranelift`) and pass it to an engine (`Universal`). We then pass the engine to
    // the store and are now ready to compile and run WebAssembly!
    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    // We then use our store and Wasm bytes to compile a `Module`.
    // A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
    let module = Module::new(&store, wasm_bytes)?;

    // Next we'll set up our `Module` so that we can execute it.

    // We define a function to act as our "env" "say_hello" function imported in the
    // Wasm program above.
    fn say_hello_world() {
        println!("Hello, world!")
    }

    // We then create an import object so that the `Module`'s imports can be satisfied.
    let import_object = imports! {
        // We use the default namespace "env".
        "env" => {
            // And call our function "say_hello".
            "say_hello" => Function::new_native(&store, say_hello_world),
        }
    };

    // We then use the `Module` and the import object to create an `Instance`.
    //
    // An `Instance` is a compiled WebAssembly module that has been set up
    // and is ready to execute.
    let instance = Instance::new(&module, &import_object)?;

    // We get the `NativeFunc` with no parameters and no results from the instance.
    //
    // Recall that the Wasm module exported a function named "run", this is getting
    // that exported function from the `Instance`.
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;

    // Finally, we call our exported Wasm function which will call our "say_hello"
    // function and return.
    run_func.call()?;

    Ok(())
}

///# Safety
#[no_mangle]
pub unsafe extern "C" fn say_something(some_string: *const u8, some_len: usize) -> SgxStatus {
    let str_slice = slice::from_raw_parts(some_string, some_len);
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word: [u8; 4] = [82, 117, 115, 116];
    // An vector
    let word_vec: Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8").as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    println!("{:?}", main());

    SgxStatus::Success
}
