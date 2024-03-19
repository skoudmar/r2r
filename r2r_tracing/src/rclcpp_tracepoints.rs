use crate::{tracetools_bindings as tp, TracingId};
use r2r_rcl::{rcl_node_t, rcl_service_t, rcl_subscription_t, rcl_timer_t};
use std::{ffi::CString, ptr::null};

// Documentation copied from github:ros2/ros2_tracing project
// TODO: Rewrite the docs according to Rust spec not C++
// TODO: Check that all references and pointers must be stable (not change location)

fn to_address<T>(t: &T) -> usize {
    t as *const T as usize
}

fn ref_to_c_void<T>(t: &T) -> *const std::ffi::c_void {
    t as *const _ as *const std::ffi::c_void
}

macro_rules! c_void {
    ($e:ident) => {
        ($e as *const std::ffi::c_void)
    };
}

/// `rclcpp_publish`
/**
 * Message publication.
 * Notes the pointer to the message being published at the `rclcpp` level.
 *
 * \param[in] message pointer to the message being published
 */
pub fn trace_publish(message: *const std::ffi::c_void) {
    unsafe {
        tp::ros_trace_rclcpp_publish(null(), message);
    }
}

/// `rclcpp_subscription_init`
/**
 * Subscription object initialisation.
 * Links the `rclcpp::*Subscription*` object to a `rcl_subscription_t` handle.
 * Needed since there could be more than 1 `rclcpp::*Subscription*` object
 * for one `rcl` subscription (e.g. when using intra-process).
 *
 * \param[in] subscription_handle
 *  pointer to the `rcl_subscription_t` handle of the subscription this object belongs to
 * \param[in] subscription pointer to this subscription object (e.g. `rclcpp::*Subscription*`)
 */
pub fn trace_subscription_init<S>(
    subscription_handle: *const rcl_subscription_t, subscription: &S,
) {
    unsafe {
        tp::ros_trace_rclcpp_subscription_init(
            c_void!(subscription_handle),
            ref_to_c_void(subscription),
        );
    }
}

/// `rclcpp_subscription_callback_added`
/**
 * Link a subscription callback object to a subscription object.
 *
 * \param[in] subscription pointer to the subscription object this callback belongs to
 * \param[in] callback pointer to this callback object (e.g. `rclcpp::AnySubscriptionCallback`)
 */
pub fn trace_subscription_callback_added<S>(subscription: &S, callback_id: usize) {
    unsafe {
        tp::ros_trace_rclcpp_subscription_callback_added(
            ref_to_c_void(subscription),
            c_void!(callback_id),
        );
    }
}

/// `rclcpp_take`
/**
 * Message taking.
 * Notes the pointer to the message being taken at the `rclcpp` level.
 *
 * \param[in] message pointer to the message being taken
 */
pub fn trace_take<M>(message: &M) {
    unsafe {
        tp::ros_trace_rclcpp_take(ref_to_c_void(message));
    }
}

/// `rclcpp_take`
/**
 * Message taking.
 * Notes the pointer to the message being taken at the `rclcpp` level.
 *
 * \param[in] message pointer to the message being taken
 */
pub fn trace_take_ptr(message: *const std::ffi::c_void) {
    unsafe {
        tp::ros_trace_rclcpp_take(message);
    }
}

/// `rclcpp_service_callback_added`
/**
 * Link a service callback object to a service.
 *
 * \param[in] service_handle
 *  pointer to the `rcl_service_t` handle of the service this callback belongs to
 * \param[in] callback pointer to this callback object (e.g. `rclcpp::AnyServiceCallback`)
 */
pub fn trace_service_callback_added(service: *const rcl_service_t, callback_id: usize) {
    unsafe { tp::ros_trace_rclcpp_service_callback_added(c_void!(service), c_void!(callback_id)) }
}

/// `rclcpp_timer_callback_added`
/**
 * Link a timer callback object to its `rcl_timer_t` handle.
 *
 * \param[in] timer_handle
 *  pointer to the `rcl_timer_t` handle of the timer this callback belongs to
 * \param[in] callback pointer to the callback object (`std::function`)
 */
pub fn trace_timer_callback_added(timer: TracingId<rcl_timer_t>, callback_id: usize) {
    unsafe {
        tp::ros_trace_rclcpp_timer_callback_added(timer.c_void(), c_void!(callback_id));
    }
}

/// `rclcpp_timer_link_node`
/**
 * Link a timer to a node.
 *
 * \param[in] timer_handle pointer to the timer's `rcl_timer_t` handle
 * \param[in] node_handle pointer to the `rcl_node_t` handle of the node the timer belongs to
 */
pub fn trace_timer_link_node(timer: TracingId<rcl_timer_t>, node: TracingId<rcl_node_t>) {
    unsafe {
        tp::ros_trace_rclcpp_timer_link_node(timer.c_void(), node.c_void());
    }
}

/// `rclcpp_callback_register`
/**
 * Register a demangled function symbol with a callback.
 *
 * \param[in] callback pointer to the callback object
 *  (e.g. `rclcpp::AnySubscriptionCallback`,
 *  `rclcpp::AnyServiceCallback`, timer `std::function`, etc.)
 * \param[in] function_symbol demangled symbol of the callback function/lambda,
 *  see \ref get_symbol()
 */
pub fn trace_callback_register(callback_id: usize, function_symbol: &str) {
    let function_symbol = CString::new(function_symbol)
        .expect("r2r tracing: Cannot convert function_symbol to CString");

    unsafe {
        tp::ros_trace_rclcpp_callback_register(c_void!(callback_id), function_symbol.as_ptr());
    }
}

/// `callback_start`
/**
 * Start of a callback.
 *
 * \param[in] callback pointer to this callback object
 *  (e.g. `rclcpp::AnySubscriptionCallback`,
 *  `rclcpp::AnyServiceCallback`, timer `std::function`, etc.)
 * \param[in] is_intra_process whether this callback is done via intra-process or not
 */
pub fn trace_callback_start(callback_id: usize, is_intra_process: bool) {
    unsafe {
        tp::ros_trace_callback_start(c_void!(callback_id), is_intra_process);
    }
}

/// `callback_end`
/**
 * End of a callback.
 *
 * \param[in] callback pointer to this callback object
 *  (e.g. `rclcpp::AnySubscriptionCallback`,
 *  `rclcpp::AnyServiceCallback`, timer `std::function`, etc.)
 */
pub fn trace_callback_end(callback_id: usize) {
    unsafe {
        tp::ros_trace_callback_end(c_void!(callback_id));
    }
}

/// `rclcpp_executor_get_next_ready`
/**
 * Notes the start time of the executor phase that gets the next executable that's ready.
 */
pub fn trace_executor_get_next_ready() {
    unsafe {
        tp::ros_trace_rclcpp_executor_get_next_ready();
    }
}

/// `rclcpp_executor_wait_for_work`
/**
 * Notes the start time of the executor phase that waits for work and notes the timeout value.
 *
 * \param[in] timeout the timeout value for the wait call
 */
pub fn trace_executor_wait_for_work(timeout: i64) {
    unsafe {
        tp::ros_trace_rclcpp_executor_wait_for_work(timeout);
    }
}

/// `rclcpp_executor_execute`
/**
 * Executable execution.
 * Notes an executable being executed using its `rcl` handle, which can be a:
 *   * timer
 *   * subscription
 *
 * \param[in] handle pointer to the `rcl` handle of the executable being executed
 */
pub fn trace_executor_execute<H>(handle: *const H) {
    unsafe {
        tp::ros_trace_rclcpp_executor_execute(c_void!(handle));
    }
}
