use std::any::type_name;
use std::ffi::CString;
use r2r_rcl::{rcl_node_t, rcl_subscription_t};
use crate::rclcpp_tracepoints_internal::ros2 as tp;

// TODO: Format the docs according to rust not c++
// TODO: All references must be stable (not change location)

fn to_address<T>(t: &T) -> usize {
    t as *const T as usize
}

/// `rclcpp_publish`
/**
 * Message publication.
 * Notes the pointer to the message being published at the `rclcpp` level.
 *
 * \param[in] message pointer to the message being published
 */
pub fn trace_publish<M>(message: &M) {
    tp::rclcpp_publish(to_address(message));
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
pub fn trace_subscription_init<S>(subscription_handle: *const rcl_subscription_t, subscription: &S) {
    tp::rclcpp_subscription_init(subscription_handle as usize, to_address(subscription));
}

/// `rclcpp_subscription_callback_added`
/**
 * Link a subscription callback object to a subscription object.
 *
 * \param[in] subscription pointer to the subscription object this callback belongs to
 * \param[in] callback pointer to this callback object (e.g. `rclcpp::AnySubscriptionCallback`)
 */
pub fn trace_subscription_callback_added<S, C>(subscription: &S, callback: &C) {
    tp::rclcpp_subscription_callback_added(to_address(subscription), to_address(callback));
}

/// `rclcpp_take`
/**
 * Message taking.
 * Notes the pointer to the message being taken at the `rclcpp` level.
 *
 * \param[in] message pointer to the message being taken
 */
pub fn trace_take<M>(message: &M) {
    tp::rclcpp_take(to_address(message));
}

/// `rclcpp_service_callback_added`
/**
 * Link a service callback object to a service.
 *
 * \param[in] service_handle
 *  pointer to the `rcl_service_t` handle of the service this callback belongs to
 * \param[in] callback pointer to this callback object (e.g. `rclcpp::AnyServiceCallback`)
 */
pub fn trace_service_callback_added<S,C>(service: &S, callback: &C) {
    tp::rclcpp_service_callback_added(to_address(service), to_address(callback));
}

/// `rclcpp_timer_callback_added`
/**
 * Link a timer callback object to its `rcl_timer_t` handle.
 *
 * \param[in] timer_handle
 *  pointer to the `rcl_timer_t` handle of the timer this callback belongs to
 * \param[in] callback pointer to the callback object (`std::function`)
 */
pub fn trace_timer_callback_added<T,C>(timer: &T, callback: &C) {
    tp::rclcpp_timer_callback_added(to_address(timer), to_address(callback));
}

/// `rclcpp_timer_link_node`
/**
 * Link a timer to a node.
 *
 * \param[in] timer_handle pointer to the timer's `rcl_timer_t` handle
 * \param[in] node_handle pointer to the `rcl_node_t` handle of the node the timer belongs to
 */
pub fn trace_timer_link_node<T>(timer: &T, node: *const rcl_node_t) {
    tp::rclcpp_timer_link_node(to_address(timer), node as usize)
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
pub fn trace_callback_register<'a, C>(callback: &C, function_symbol: impl Into<Option<&'a str>>) {
    let function_symbol = CString::new(
        function_symbol
            .into()
            .unwrap_or_else(|| type_name::<C>()))
        .expect("r2r tracing: Cannot convert function_symbol to CString");

    tp::rclcpp_callback_register(to_address(callback), function_symbol.as_c_str());
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
pub fn trace_callback_start<C>(callback: &C, is_intra_process: bool) {
    let is_intra_process = match is_intra_process {
        true => 1,
        false => 0,
    };

    tp::callback_start(to_address(callback), is_intra_process);
}

/// `callback_end`
/**
 * End of a callback.
 *
 * \param[in] callback pointer to this callback object
 *  (e.g. `rclcpp::AnySubscriptionCallback`,
 *  `rclcpp::AnyServiceCallback`, timer `std::function`, etc.)
 */
pub fn trace_callback_end<C>(callback: &C) {
    tp::callback_end(to_address(callback));
}

/// `rclcpp_executor_get_next_ready`
/**
 * Notes the start time of the executor phase that gets the next executable that's ready.
 */
pub fn trace_executor_get_next_ready() {
    tp::rclcpp_executor_get_next_ready();
}

/// `rclcpp_executor_wait_for_work`
/**
 * Notes the start time of the executor phase that waits for work and notes the timeout value.
 *
 * \param[in] timeout the timeout value for the wait call
 */
pub fn trace_executor_wait_for_work(timeout: i64) {
    tp::rclcpp_executor_wait_for_work(timeout);
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
    tp::rclcpp_executor_execute(handle as usize);
}




