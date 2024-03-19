use lttng_ust::import_tracepoints;

import_tracepoints!(concat!(env!("OUT_DIR"), "/r2r_tracepoints.rs"), r2r_tracepoints_internal);

mod rclcpp_tracepoints;
pub use rclcpp_tracepoints::*;

mod callback;
mod r2r_tracepoints;
mod tracetools_bindings;

pub use callback::Callback;
pub use r2r_tracepoints::*;
