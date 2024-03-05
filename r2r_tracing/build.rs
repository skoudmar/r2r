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
    generate_rclcpp_tracepoints();
    generate_r2r_tracepoints();
}

fn generate_rclcpp_tracepoints() {
    let mut ros2 = Provider::new("ros2");

    create_tracepoint!(ros2::rclcpp_publish(
        message: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_subscription_init(
        subscription_handle: CTFType::IntegerHex(CIntegerType::USize),
        subscription: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_subscription_callback_added(
        subscription: CTFType::IntegerHex(CIntegerType::USize),
        callback: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_take(
        message: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_service_callback_added(
        service_handle: CTFType::IntegerHex(CIntegerType::USize),
        callback: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_timer_callback_added(
        timer_handle: CTFType::IntegerHex(CIntegerType::USize),
        callback: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_timer_link_node(
        timer_handle: CTFType::IntegerHex(CIntegerType::USize),
        node_handle: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_callback_register(
        callback: CTFType::IntegerHex(CIntegerType::USize),
        symbol: CTFType::String, // must be null terminated
        // TODO: check if CTFType::SequenceText is better
    ));
    create_tracepoint!(ros2::callback_start(
        callback: CTFType::IntegerHex(CIntegerType::USize),
        is_intra_process: CTFType::Integer(CIntegerType::U8), // should be boolean
    ));
    create_tracepoint!(ros2::callback_end(
        callback: CTFType::IntegerHex(CIntegerType::USize),
    ));
    create_tracepoint!(ros2::rclcpp_executor_get_next_ready());
    create_tracepoint!(ros2::rclcpp_executor_wait_for_work(
        timeout: CTFType::Integer(CIntegerType::I64),
    ));
    create_tracepoint!(ros2::rclcpp_executor_execute(
        handle: CTFType::IntegerHex(CIntegerType::USize),
    ));

    Generator::default()
        .generated_lib_name("r2r_tracepoints_rclcpp")
        .register_provider(ros2)
        .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("rclcpp_tracepoints"))
        .generate()
        .expect("Unable to generate tracepoint bindings for r2r/rclcpp");
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

    Generator::default()
        .generated_lib_name("r2r_tracepoints_r2r")
        .register_provider(r2r)
        .output_file_name(PathBuf::from(env::var("OUT_DIR").unwrap()).join("r2r_tracepoints"))
        .generate()
        .expect("Unable to generate tracepoint bindings for r2r");
}
