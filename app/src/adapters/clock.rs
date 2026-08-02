use focus_flow_core::Clock;

#[derive(Clone, Copy, Default)]
pub struct AppClock;

impl Clock for AppClock {
    fn now_ms(&self) -> u64 {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            js_sys::Date::now().max(0.0) as u64
        }

        #[cfg(not(any(feature = "csr", feature = "hydrate")))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_millis() as u64)
                .unwrap_or(0)
        }
    }
}
