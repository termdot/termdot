pub mod event_handle;
pub mod events;

use event_handle::EventHandle;
use events::Events;
use std::{
    cell::RefCell,
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
    ptr::NonNull,
};
use tmui::{
    prelude::nohash_hasher::IntMap,
    tlib::{log::warn, nonnull_mut, ptr_ref},
};

thread_local! {
    static EVENT_BUS: RefCell<EventBus> = RefCell::new(EventBus::default());
}

#[derive(Default)]
pub struct EventBus {
    register: IntMap<u64, Vec<Option<NonNull<dyn EventHandle>>>>,
    deferred_events: VecDeque<Events>,
}

impl EventBus {
    pub fn register(handle: *mut dyn EventHandle) {
        EVENT_BUS.with(|rf| {
            let mut event_bus = rf.borrow_mut();

            for listen in ptr_ref!(handle).listen() {
                let mut hasher = DefaultHasher::default();
                listen.hash(&mut hasher);
                let k = hasher.finish();

                event_bus
                    .register
                    .entry(k)
                    .or_default()
                    .push(NonNull::new(handle));
            }
        })
    }

    #[inline]
    pub fn push(e: Events) {
        EVENT_BUS.with(|rf| {
            rf.borrow_mut().push_inner(e);
        });
    }

    #[inline]
    pub fn push_deferred(e: Events) {
        EVENT_BUS.with(|rf| {
            rf.borrow_mut().deferred_events.push_back(e);
        });
    }

    #[inline]
    pub fn process_deferred_evts() {
        EVENT_BUS.with(|rf| {
            let mut event_bus = rf.borrow_mut();
            while let Some(e) = event_bus.deferred_events.pop_front() {
                event_bus.push_inner(e)
            }
        });
    }

    fn push_inner(&mut self, e: Events) {
        let mut hasher = DefaultHasher::default();
        e.ty().hash(&mut hasher);
        let k = hasher.finish();

        if let Some(registers) = self.register.get_mut(&k) {
            for handle in registers.iter_mut() {
                let handle = nonnull_mut!(handle);
                handle.handle(&e);
            }
        } else {
            warn!("[EventBus::push] Event `{:?}` has no registers.", e.ty(),);
        }
    }
}
