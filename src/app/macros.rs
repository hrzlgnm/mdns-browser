#[macro_export]
macro_rules! log_fn {
    ($name:expr, $body:block) => {{
        log::debug!("-> {}", $name);
        let result = { $body };
        log::debug!("<- {}", $name);
        result
    }};
}
