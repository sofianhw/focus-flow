use std::cell::RefCell;
use std::rc::Rc;

use leptos::prelude::on_cleanup;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Default)]
pub struct TimerScheduler {
    handle: Rc<RefCell<Option<TimerHandle>>>,
}

impl TimerScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync(&self, running: bool, callback: impl Fn() + 'static) {
        if running && self.handle.borrow().is_none() {
            *self.handle.borrow_mut() = TimerHandle::start(callback);
        } else if !running {
            self.cancel();
        }
    }

    pub fn cancel(&self) {
        self.handle.borrow_mut().take();
    }

    pub fn install_cleanup(&self) {
        let scheduler = SendWrapper::new(self.clone());
        on_cleanup(move || scheduler.cancel());
    }
}

struct TimerHandle {
    id: i32,
    _callback: Closure<dyn FnMut()>,
}

impl TimerHandle {
    fn start(callback: impl Fn() + 'static) -> Option<Self> {
        let window = web_sys::window()?;
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
        let id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                1_000,
            )
            .ok()?;
        Some(Self { id, _callback: callback })
    }
}

impl Drop for TimerHandle {
    fn drop(&mut self) {
        if let Some(window) = web_sys::window() {
            window.clear_interval_with_handle(self.id);
        }
    }
}
