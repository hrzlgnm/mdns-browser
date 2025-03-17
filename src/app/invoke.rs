use tauri_sys::core::invoke;

use crate::log_fn;

pub async fn invoke_no_args(cmd: &str) {
    log_fn!(format!("invoke_no_args(`{}`)", cmd), {
        let _ = invoke::<()>(cmd, &()).await;
    })
}
