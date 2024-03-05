
use lttng_ust::import_tracepoints;

import_tracepoints!(concat!(env!("OUT_DIR"), "/rclcpp_tracepoints"), rclcpp_tracepoints_internal);
import_tracepoints!(concat!(env!("OUT_DIR"), "/r2r_tracepoints"), r2r_tracepoints_internal);

mod rclcpp_tracepoints;
pub use rclcpp_tracepoints::*;

mod r2r_tracepoints;
pub use r2r_tracepoints::*;
