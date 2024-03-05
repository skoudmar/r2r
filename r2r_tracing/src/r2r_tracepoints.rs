use std::time::Duration;
use r2r_rcl::rcl_node_t;
use crate::r2r_tracepoints_internal::r2r as tp;

pub fn trace_spin_start(node: *const rcl_node_t, timeout: Duration) {
    let timeout_s = timeout.as_secs();
    let timeout_ns = timeout.subsec_nanos();

    tp::spin_start(node as usize, timeout_s, timeout_ns);
}

pub fn trace_spin_end(node: *const rcl_node_t) {
    tp::spin_end(node as usize);
}

pub fn trace_spin_timeout(node: *const rcl_node_t) {
    tp::spin_timeout(node as usize);
}