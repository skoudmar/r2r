use lttng_ust_generate::{CIntegerType, CTFType, Generator, Provider};
use std::env;
use std::path::PathBuf;

macro_rules! create_tracepoint {
    ($provider:ident::$name:ident($($arg_name:ident: $arg_lttng_type:expr),* $(,)?)) => {
        $provider.create_class(concat!(stringify!($name), "_class"))
            $(
                .add_field(stringify!($arg_name), $arg_lttng_type)
            )*
        .instantiate(stringify!($name))
    };
}

fn main() {
    tracetools_bindgen();
    generate_r2r_tracepoints();
}

fn tracetools_bindgen() {
    let bindings = r2r_common::setup_bindgen_builder()
        .header("src/tracetools_wrapper.h")
        .allowlist_function("ros_trace_.*")
        .generate_comments(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate tracetools bindings");

    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=tracetools");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("tracetools_bindings.rs"))
        .expect("Couldn't write tracetools bindings!");
}

fn generate_r2r_tracepoints() {
    let mut r2r = Provider::new("r2r");

    create_tracepoint!(r2r::spin_start(
        node_handle: CTFType::IntegerHex(CIntegerType::USize),
        timeout_s: CTFType::Integer(CIntegerType::U64),
        timeout_ns: CTFType::Integer(CIntegerType::U32),
    ));
    create_tracepoint!(r2r::spin_end(
        node_handle: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(r2r::spin_timeout(
        node_handle: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(r2r::update_time(
        subscriber: CTFType::IntegerHex(CIntegerType::USize),
        time_s: CTFType::Integer(CIntegerType::I32),
        time_ns: CTFType::Integer(CIntegerType::U32),
    ));

    Generator::default()
        .generated_lib_name("r2r_tracepoints_r2r")
        .register_provider(r2r)
        .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("r2r_tracepoints.rs"))
        .generate()
        .expect("Unable to generate tracepoint bindings for r2r");
}
