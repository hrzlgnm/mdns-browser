#[macro_export]
macro_rules! log_fn {
    ($name:expr, $body:block) => {{
        #[cfg(debug_assertions)]
        log::debug!("-> {}", $name);

        let result = { $body };

        #[cfg(debug_assertions)]
        log::debug!("<- {}", $name);

        result
    }};
}
