use crate::{
    trace_callback_register, trace_service_callback_added, trace_subscription_callback_added,
    trace_timer_callback_added,
};
use r2r_rcl::{rcl_service_t, rcl_timer_t};
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub struct Callback<F, M>
where
    F: FnMut(M),
{
    func: F,
    id: usize,
    msg_type: PhantomData<M>,
}

impl<F, M> Callback<F, M>
where
    F: FnMut(M),
{
    fn gen_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Relaxed)
    }
    fn new(callback: F, id: usize) -> Self {
        trace_callback_register(id, type_name::<F>());

        Self {
            func: callback,
            id,
            msg_type: Default::default(),
        }
    }

    pub fn new_service(service: *const rcl_service_t, callback: F) -> Self {
        let id = Self::gen_id();
        trace_service_callback_added(service, id);

        Self::new(callback, id)
    }

    pub fn new_timer(timer: *const rcl_timer_t, callback: F) -> Self {
        let id = Self::gen_id();
        trace_timer_callback_added(timer, id);

        Self::new(callback, id)
    }

    pub fn new_subscription<S>(subscriber: &S, callback: F) -> Self {
        let id = Self::gen_id();
        trace_subscription_callback_added(subscriber, id);

        Self::new(callback, id)
    }

    pub fn call(&mut self, msg: M) {
        crate::trace_callback_start(self.id, false);
        (self.func)(msg);
        crate::trace_callback_end(self.id);
    }
}
