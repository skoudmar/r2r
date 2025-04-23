use std::{
    ffi::c_void,
    future::{self, Future},
};

use futures::{channel::mpsc::Receiver, stream::FusedStream, Stream, StreamExt as _};
use r2r_rcl::rcl_service_t;

use crate::{Callback, TracingId};

#[derive(Debug)]
pub struct StreamWithTracingData<T> {
    stream: Receiver<T>,
    tracing_id: TracingIdWithType,
}

#[derive(Debug, Clone, Copy)]
enum TracingIdWithType {
    /// The type of subscription is `c_void` because the actual type would
    /// be a generic Rust subscriber not `rcl_subscription_t`.
    Subscription(TracingId<c_void>),
    Service(TracingId<rcl_service_t>),
}

impl<T> StreamWithTracingData<T> {
    /// Converts the stream into a future that calls the provided callback for each message.
    ///
    /// If `tracing` feature is enabled, the callback execution will be traced.
    pub fn traced_callback<C>(self, callback: C) -> impl Future<Output = ()> + Unpin
    where
        C: FnMut(T),
    {
        let mut callback_wrapper = match self.tracing_id {
            TracingIdWithType::Subscription(id) => Callback::new_subscription(id, callback),
            TracingIdWithType::Service(id) => Callback::new_service(id, callback),
        };

        self.stream.for_each(move |msg| {
            callback_wrapper.call(msg);
            future::ready(())
        })
    }
}

impl<T> Stream for StreamWithTracingData<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        this.stream.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

impl<T> FusedStream for StreamWithTracingData<T> {
    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

/// A builder for `StreamWithTracingData`.
///
/// This struct exists to allow creation of `StreamWithTracingData` without polling its public API.
/// It is used internally by r2r and should not be reexported.
pub struct StreamWithTracingDataBuilder;

impl StreamWithTracingDataBuilder {
    #[must_use]
    #[allow(
        clippy::not_unsafe_ptr_arg_deref,
        reason = "The pointer is not dereferenced"
    )]
    pub fn build_service<T>(
        stream: Receiver<T>, service_id: TracingId<rcl_service_t>,
    ) -> StreamWithTracingData<T> {
        let tracing_id = TracingIdWithType::Service(service_id);
        StreamWithTracingData { stream, tracing_id }
    }

    #[must_use]
    pub fn build_subscription<T>(
        stream: Receiver<T>, subscription_id: TracingId<c_void>,
    ) -> StreamWithTracingData<T> {
        let tracing_id = TracingIdWithType::Subscription(subscription_id);
        StreamWithTracingData { stream, tracing_id }
    }
}
